#!/usr/bin/env python3
import argparse
import dataclasses
import os
import pathlib
import re
import shutil
import subprocess
import tempfile
import urllib.request

QEMU_REF_DEFAULT = "v9.0.0"
QEMU_BASE = "https://raw.githubusercontent.com/qemu/qemu/{ref}/target/hexagon/"

IMPORT_FILES = [
    "allidefs.def",
    "branch.idef",
    "ldst.idef",
    "compare.idef",
    "mpy.idef",
    "alu.idef",
    "float.idef",
    "shift.idef",
    "system.idef",
    "subinsns.idef",
    "allext.idef",
    "macros.def",
    "allext_macros.def",
    "allextenc.def",
    "encode.def",
    "encode_pp.def",
    "encode_subinsn.def",
]

MMVEC_FILES = [
    "mmvec/encode_ext.def",
    "mmvec/ext.idef",
    "mmvec/macros.def",
]

ROOT_FILES = [
    "gen_semantics.c",
]

ISA_ATTRS = {
    "A_ARCHV2": 4,
    "A_ARCHV3": 4,
    "A_ARCHV4": 4,
    "A_ARCHV5": 5,
    "A_ARCHV55": 55,
    "A_ARCHV60": 60,
    "A_ARCHV62": 62,
    "A_ARCHV65": 65,
    "A_ARCHV66": 66,
    "A_ARCHV67": 67,
    "A_ARCHV68": 68,
    "A_ARCHV69": 69,
}

ENC_MACRO_HEADER = """\
#define DEF_ENC32(TAG, ENCSTR) ENC32(TAG, ENCSTR)
#define DEF_ENC_SUBINSN(TAG, CLASS, ENCSTR) ENCSUB(TAG, CLASS, ENCSTR)
#define DEF_EXT_ENC(TAG, CLASS, ENCSTR) ENCEXT(TAG, CLASS, ENCSTR)
"""


@dataclasses.dataclass(frozen=True)
class EncodingDef:
    tag: str
    enc: str
    enc_class: str
    bits_len: int
    mask: int
    value: int
    fields: dict


def fetch_file(url: str, dest: pathlib.Path, force: bool) -> None:
    if dest.exists() and not force:
        return
    dest.parent.mkdir(parents=True, exist_ok=True)
    with urllib.request.urlopen(url) as resp:
        dest.write_bytes(resp.read())


def ensure_qemu_files(ref: str, out_dir: pathlib.Path, force: bool) -> None:
    out_dir.mkdir(parents=True, exist_ok=True)
    ref_path = out_dir / "REF"
    if ref_path.exists() and not force:
        existing = ref_path.read_text().strip()
        if existing != ref:
            raise SystemExit(
                f"qemu ref mismatch: have {existing}, want {ref}. Use --force to refresh."
            )
    ref_path.write_text(ref)

    base = QEMU_BASE.format(ref=ref)
    for name in ROOT_FILES:
        fetch_file(base + name, out_dir / name, force)

    imported = out_dir / "imported"
    for name in IMPORT_FILES:
        fetch_file(base + "imported/" + name, imported / name, force)
    for name in MMVEC_FILES:
        fetch_file(base + "imported/" + name, imported / name, force)


def run_gen_semantics(qemu_dir: pathlib.Path, out_dir: pathlib.Path) -> pathlib.Path:
    gen_semantics_c = qemu_dir / "gen_semantics.c"
    gen_semantics_bin = out_dir / "gen_semantics"
    cmd = [
        "cc",
        "-O2",
        "-I",
        str(qemu_dir),
        "-o",
        str(gen_semantics_bin),
        str(gen_semantics_c),
    ]
    subprocess.run(cmd, check=True)
    semantics_path = out_dir / "semantics_generated.pyinc"
    subprocess.run([str(gen_semantics_bin), str(semantics_path)], check=True)
    return semantics_path


def parse_quoted(line: str) -> str:
    start = line.find('"')
    if start < 0:
        return ""
    end = line.find('"', start + 1)
    if end < 0:
        return ""
    return line[start + 1 : end]


def parse_attributes(semantics_path: pathlib.Path) -> dict:
    attrs = {}
    lines = semantics_path.read_text().splitlines()
    idx = 0
    while idx < len(lines):
        if lines[idx].lstrip().startswith("ATTRIBUTES("):
            if idx + 2 >= len(lines):
                break
            tag_line = lines[idx + 1].strip()
            attr_line = lines[idx + 2].strip()
            tag = parse_quoted(tag_line)
            attr_blob = parse_quoted(attr_line)
            attr_blob = (
                attr_blob.replace("ATTRIBS", "")
                .replace("(", "")
                .replace(")", "")
                .strip()
            )
            if attr_blob:
                attr_list = [part.strip() for part in attr_blob.split(",") if part.strip()]
            else:
                attr_list = []
            attrs[tag] = attr_list
        idx += 1
    return attrs


def preprocess_encodings(imported_dir: pathlib.Path) -> str:
    with tempfile.TemporaryDirectory() as tmpdir:
        tmp = pathlib.Path(tmpdir)
        macro_path = tmp / "enc_macros.h"
        macro_path.write_text(ENC_MACRO_HEADER)
        enc_def = imported_dir / "encode.def"
        cmd = [
            "cc",
            "-E",
            "-P",
            "-x",
            "c",
            "-I",
            str(imported_dir),
            "-include",
            str(macro_path),
            str(enc_def),
        ]
        proc = subprocess.run(cmd, check=True, capture_output=True, text=True)
        return proc.stdout


def iter_macro_calls(text: str, macro: str, has_class: bool):
    needle = macro + "("
    idx = 0
    while True:
        start = text.find(needle, idx)
        if start < 0:
            break
        idx = start + len(needle)
        while idx < len(text) and text[idx].isspace():
            idx += 1
        tag_start = idx
        while idx < len(text) and text[idx] != ",":
            idx += 1
        tag = text[tag_start:idx].strip()
        if idx >= len(text) or text[idx] != ",":
            continue
        idx += 1
        enc_class = None
        if has_class:
            while idx < len(text) and text[idx].isspace():
                idx += 1
            cls_start = idx
            while idx < len(text) and text[idx] != ",":
                idx += 1
            enc_class = text[cls_start:idx].strip()
            if idx >= len(text) or text[idx] != ",":
                continue
            idx += 1
        parts = []
        while idx < len(text):
            while idx < len(text) and text[idx].isspace():
                idx += 1
            if idx >= len(text):
                break
            if text[idx] == ")":
                idx += 1
                break
            if text[idx] == '"':
                idx += 1
                lit_start = idx
                while idx < len(text) and text[idx] != '"':
                    if text[idx] == "\\":
                        idx += 2
                        continue
                    idx += 1
                parts.append(text[lit_start:idx])
                if idx < len(text) and text[idx] == '"':
                    idx += 1
                continue
            idx += 1
        enc = "".join(parts)
        if enc:
            yield tag, enc, enc_class


def parse_encoding_defs(text: str) -> list[EncodingDef]:
    encs = []
    for tag, enc, _cls in iter_macro_calls(text, "ENC32", has_class=False):
        encs.append(build_encoding(tag, enc, "NORMAL"))
    for tag, enc, cls in iter_macro_calls(text, "ENCSUB", has_class=True):
        encs.append(build_encoding(tag, enc, cls))
    for tag, enc, cls in iter_macro_calls(text, "ENCEXT", has_class=True):
        encs.append(build_encoding(tag, enc, cls))
    return encs


def build_encoding(tag: str, enc: str, enc_class: str) -> EncodingDef:
    pattern = enc.replace(" ", "")
    bits_len = len(pattern)
    if bits_len not in (32, 13):
        raise SystemExit(f"unexpected pattern length {bits_len} for {tag}: {enc}")
    mask = 0
    value = 0
    fields: dict[str, list[int]] = {}
    for idx, ch in enumerate(pattern):
        bit = bits_len - 1 - idx
        if ch == "0" or ch == "1":
            mask |= 1 << bit
            if ch == "1":
                value |= 1 << bit
        elif ch == "-" or ch == ".":
            continue
        else:
            if ch == "P":
                continue
            fields.setdefault(ch, []).append(bit)
    return EncodingDef(
        tag=tag,
        enc=enc,
        enc_class=enc_class,
        bits_len=bits_len,
        mask=mask,
        value=value,
        fields=fields,
    )


def sanitize_tag(tag: str) -> str:
    # Rust enum variants can't include '.' or '-' etc. Replace invalid chars with '_'.
    sanitized = re.sub(r"[^A-Za-z0-9_]", "_", tag)
    if not sanitized[0].isalpha() and sanitized[0] != "_":
        sanitized = "_" + sanitized
    return sanitized


def write_rust(
    out_path: pathlib.Path,
    encs: list[EncodingDef],
    attrs: dict,
) -> None:
    # Build opcode list
    tags = sorted({enc.tag for enc in encs})
    variant_map = {}
    variants = []
    for tag in tags:
        var = sanitize_tag(tag)
        if var in variant_map:
            raise SystemExit(f"enum variant collision for {tag} -> {var}")
        variant_map[tag] = var
        variants.append(var)

    # Min ISA version per opcode
    min_versions = []
    for tag in tags:
        v = 4
        for attr in attrs.get(tag, []):
            if attr in ISA_ATTRS:
                v = max(v, ISA_ATTRS[attr])
        min_versions.append(v)

    # Dedup bit arrays
    bits_pool = {}
    bits_defs = []

    def bits_name(bits: list[int]) -> str:
        key = tuple(bits)
        if key in bits_pool:
            return bits_pool[key]
        name = f"BITS_{len(bits_pool)}"
        bits_pool[key] = name
        bits_defs.append((name, bits))
        return name

    # Build field pool and enc data
    field_pool = []
    enc_data = []

    for enc in encs:
        fields = []
        for letter in sorted(enc.fields.keys()):
            bits = enc.fields[letter]
            name = bits_name(bits)
            fields.append((letter, name))
        start = len(field_pool)
        for letter, name in fields:
            field_pool.append((letter, name))
        enc_data.append((enc, start, len(fields)))

    # Group encodings
    word_encs = []
    sub_encs = []
    for enc, start, length in enc_data:
        if enc.bits_len == 32:
            word_encs.append((enc, start, length))
        else:
            sub_encs.append((enc, start, length))

    def fixed_bits(enc: EncodingDef) -> int:
        return bin(enc.mask).count("1")

    # Group word encs by iclass
    iclass_groups: dict[int, list] = {i: [] for i in range(16)}
    misc_group = []
    for enc, start, length in word_encs:
        if enc.mask & 0xF0000000 == 0xF0000000:
            iclass = (enc.value >> 28) & 0xF
            iclass_groups[iclass].append((enc, start, length))
        else:
            misc_group.append((enc, start, length))

    for group in iclass_groups.values():
        group.sort(key=lambda item: (-fixed_bits(item[0]), item[0].tag))
    misc_group.sort(key=lambda item: (-fixed_bits(item[0]), item[0].tag))

    # Group sub encs by class
    sub_groups: dict[str, list] = {}
    for enc, start, length in sub_encs:
        sub_groups.setdefault(enc.enc_class, []).append((enc, start, length))
    for group in sub_groups.values():
        group.sort(key=lambda item: (-fixed_bits(item[0]), item[0].tag))

    def rust_class(name: str) -> str:
        if name == "NORMAL":
            return "EncClass::Normal"
        if name == "SUBINSN_A":
            return "EncClass::SubinsnA"
        if name == "SUBINSN_L1":
            return "EncClass::SubinsnL1"
        if name == "SUBINSN_L2":
            return "EncClass::SubinsnL2"
        if name == "SUBINSN_S1":
            return "EncClass::SubinsnS1"
        if name == "SUBINSN_S2":
            return "EncClass::SubinsnS2"
        if name == "EXT_mmvec":
            return "EncClass::ExtMmvec"
        raise SystemExit(f"unknown encoding class {name}")

    lines = []
    lines.append("// AUTO-GENERATED. DO NOT EDIT BY HAND.\n")
    lines.append("// Generated by tools/hexagon/gen_iset.py\n\n")

    lines.append("#[allow(non_camel_case_types)]\n")
    lines.append("#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n")
    lines.append("pub enum Opcode {\n")
    for var in variants:
        lines.append(f"    {var},\n")
    lines.append("}\n\n")

    lines.append("pub const OPCODE_NAMES: [&str; ")
    lines.append(str(len(tags)))
    lines.append("] = [\n")
    for tag in tags:
        lines.append(f"    \"{tag}\",\n")
    lines.append("];\n\n")

    lines.append("pub const OPCODE_MIN_VERSION: [u16; ")
    lines.append(str(len(tags)))
    lines.append("] = [\n")
    for v in min_versions:
        lines.append(f"    {v}u16,\n")
    lines.append("];\n\n")

    lines.append("#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n")
    lines.append("pub enum EncClass {\n")
    lines.append("    Normal,\n")
    lines.append("    SubinsnA,\n")
    lines.append("    SubinsnL1,\n")
    lines.append("    SubinsnL2,\n")
    lines.append("    SubinsnS1,\n")
    lines.append("    SubinsnS2,\n")
    lines.append("    ExtMmvec,\n")
    lines.append("}\n\n")

    lines.append("#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n")
    lines.append("pub struct FieldDesc {\n")
    lines.append("    pub letter: u8,\n")
    lines.append("    pub bits: &'static [u8],\n")
    lines.append("}\n\n")

    lines.append("#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n")
    lines.append("pub struct Encoding {\n")
    lines.append("    pub opcode: Opcode,\n")
    lines.append("    pub mask: u32,\n")
    lines.append("    pub value: u32,\n")
    lines.append("    pub class: EncClass,\n")
    lines.append("    pub fields_start: u32,\n")
    lines.append("    pub fields_len: u32,\n")
    lines.append("}\n\n")

    for name, bits in bits_defs:
        lines.append(f"const {name}: [u8; {len(bits)}] = [")
        lines.append(", ".join(str(b) for b in bits))
        lines.append("];\n")

    lines.append("\npub const FIELD_POOL: &[FieldDesc] = &[\n")
    for letter, bits_name_ref in field_pool:
        lines.append(
            f"    FieldDesc {{ letter: b'{letter}', bits: &{bits_name_ref} }},\n"
        )
    lines.append("];\n\n")

    for iclass in range(16):
        group = iclass_groups[iclass]
        lines.append(f"pub const ENCODINGS_ICLASS_{iclass:X}: &[Encoding] = &[\n")
        for enc, start, length in group:
            lines.append(
                f"    Encoding {{ opcode: Opcode::{variant_map[enc.tag]}, mask: 0x{enc.mask:08x}, value: 0x{enc.value:08x}, class: {rust_class(enc.enc_class)}, fields_start: {start}u32, fields_len: {length}u32 }},\n"
            )
        lines.append("];\n\n")

    lines.append("pub const ENCODINGS_MISC: &[Encoding] = &[\n")
    for enc, start, length in misc_group:
        lines.append(
            f"    Encoding {{ opcode: Opcode::{variant_map[enc.tag]}, mask: 0x{enc.mask:08x}, value: 0x{enc.value:08x}, class: {rust_class(enc.enc_class)}, fields_start: {start}u32, fields_len: {length}u32 }},\n"
        )
    lines.append("];\n\n")

    lines.append("pub const ENCODINGS_BY_ICLASS: [&[Encoding]; 16] = [\n")
    for iclass in range(16):
        lines.append(f"    ENCODINGS_ICLASS_{iclass:X},\n")
    lines.append("];\n\n")

    sub_class_map = {
        "SUBINSN_A": "SUBINSN_A",
        "SUBINSN_L1": "SUBINSN_L1",
        "SUBINSN_L2": "SUBINSN_L2",
        "SUBINSN_S1": "SUBINSN_S1",
        "SUBINSN_S2": "SUBINSN_S2",
    }

    for cls_key, const_name in sub_class_map.items():
        group = sub_groups.get(cls_key, [])
        lines.append(f"pub const {const_name}: &[Encoding] = &[\n")
        for enc, start, length in group:
            lines.append(
                f"    Encoding {{ opcode: Opcode::{variant_map[enc.tag]}, mask: 0x{enc.mask:08x}, value: 0x{enc.value:08x}, class: {rust_class(enc.enc_class)}, fields_start: {start}u32, fields_len: {length}u32 }},\n"
            )
        lines.append("];\n\n")

    out_path.write_text("".join(lines))


def main() -> None:
    parser = argparse.ArgumentParser(description="Generate Hexagon opcode tables from QEMU defs")
    parser.add_argument("--ref", default=QEMU_REF_DEFAULT, help="QEMU git ref")
    parser.add_argument(
        "--qemu-dir",
        default="tools/hexagon/qemu",
        help="Directory for fetched QEMU files",
    )
    parser.add_argument(
        "--out",
        default="src/backend/emulator/hexagon/opcode_generated.rs",
        help="Rust output path",
    )
    parser.add_argument("--force", action="store_true", help="Refetch QEMU files")
    args = parser.parse_args()

    qemu_dir = pathlib.Path(args.qemu_dir)
    ensure_qemu_files(args.ref, qemu_dir, args.force)

    build_dir = qemu_dir / "build"
    if build_dir.exists() and args.force:
        shutil.rmtree(build_dir)
    build_dir.mkdir(parents=True, exist_ok=True)

    semantics_path = run_gen_semantics(qemu_dir, build_dir)
    attrs = parse_attributes(semantics_path)

    enc_text = preprocess_encodings(qemu_dir / "imported")
    encs = parse_encoding_defs(enc_text)

    out_path = pathlib.Path(args.out)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    write_rust(out_path, encs, attrs)


if __name__ == "__main__":
    main()
