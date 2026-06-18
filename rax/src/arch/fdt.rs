//! Minimal flattened device tree (DTB) writer.
//!
//! Produces a device tree blob (format version 17) good enough for the Linux
//! ARM64 boot protocol: begin/end nodes, properties (raw bytes, u32/u64
//! cells, strings, string lists), automatic string-table deduplication, and
//! an empty memory reservation map. All multi-byte values are big-endian per
//! the DTB specification.

use std::collections::HashMap;

const FDT_MAGIC: u32 = 0xD00D_FEED;
const FDT_VERSION: u32 = 17;
const FDT_LAST_COMP_VERSION: u32 = 16;

const FDT_BEGIN_NODE: u32 = 0x1;
const FDT_END_NODE: u32 = 0x2;
const FDT_PROP: u32 = 0x3;
const FDT_END: u32 = 0x9;

const HEADER_LEN: usize = 40;

/// Incremental DTB builder.
pub struct FdtBuilder {
    structure: Vec<u8>,
    strings: Vec<u8>,
    string_offsets: HashMap<String, u32>,
    open_nodes: usize,
}

impl FdtBuilder {
    pub fn new() -> Self {
        FdtBuilder {
            structure: Vec::new(),
            strings: Vec::new(),
            string_offsets: HashMap::new(),
            open_nodes: 0,
        }
    }

    fn push_token(&mut self, token: u32) {
        self.structure.extend_from_slice(&token.to_be_bytes());
    }

    fn pad_structure(&mut self) {
        while self.structure.len() % 4 != 0 {
            self.structure.push(0);
        }
    }

    fn string_offset(&mut self, name: &str) -> u32 {
        if let Some(&off) = self.string_offsets.get(name) {
            return off;
        }
        let off = self.strings.len() as u32;
        self.strings.extend_from_slice(name.as_bytes());
        self.strings.push(0);
        self.string_offsets.insert(name.to_string(), off);
        off
    }

    /// Open a node. The root node uses the empty name `""`.
    pub fn begin_node(&mut self, name: &str) {
        self.push_token(FDT_BEGIN_NODE);
        self.structure.extend_from_slice(name.as_bytes());
        self.structure.push(0);
        self.pad_structure();
        self.open_nodes += 1;
    }

    /// Close the most recently opened node.
    pub fn end_node(&mut self) {
        debug_assert!(self.open_nodes > 0, "end_node without begin_node");
        self.push_token(FDT_END_NODE);
        self.open_nodes = self.open_nodes.saturating_sub(1);
    }

    /// Raw property bytes.
    pub fn prop(&mut self, name: &str, data: &[u8]) {
        let name_off = self.string_offset(name);
        self.push_token(FDT_PROP);
        self.structure
            .extend_from_slice(&(data.len() as u32).to_be_bytes());
        self.structure.extend_from_slice(&name_off.to_be_bytes());
        self.structure.extend_from_slice(data);
        self.pad_structure();
    }

    /// Empty (boolean) property.
    pub fn prop_empty(&mut self, name: &str) {
        self.prop(name, &[]);
    }

    /// Single u32 cell.
    pub fn prop_u32(&mut self, name: &str, value: u32) {
        self.prop(name, &value.to_be_bytes());
    }

    /// u64 as two cells.
    pub fn prop_u64(&mut self, name: &str, value: u64) {
        self.prop(name, &value.to_be_bytes());
    }

    /// Array of u32 cells.
    pub fn prop_cells(&mut self, name: &str, cells: &[u32]) {
        let mut data = Vec::with_capacity(cells.len() * 4);
        for c in cells {
            data.extend_from_slice(&c.to_be_bytes());
        }
        self.prop(name, &data);
    }

    /// NUL-terminated string.
    pub fn prop_str(&mut self, name: &str, value: &str) {
        let mut data = Vec::with_capacity(value.len() + 1);
        data.extend_from_slice(value.as_bytes());
        data.push(0);
        self.prop(name, &data);
    }

    /// List of NUL-terminated strings (e.g. multiple `compatible` entries).
    pub fn prop_str_list(&mut self, name: &str, values: &[&str]) {
        let mut data = Vec::new();
        for v in values {
            data.extend_from_slice(v.as_bytes());
            data.push(0);
        }
        self.prop(name, &data);
    }

    /// Finalize into a DTB.
    pub fn finish(mut self) -> Vec<u8> {
        debug_assert_eq!(self.open_nodes, 0, "unclosed node at finish");
        self.push_token(FDT_END);

        // Layout: header | mem rsvmap (empty terminator) | struct | strings.
        // Pad the strings block so the blob (and totalsize) stay 4-aligned.
        while self.strings.len() % 4 != 0 {
            self.strings.push(0);
        }
        let rsvmap_off = HEADER_LEN;
        let rsvmap_len = 16; // single (0, 0) terminator entry
        let struct_off = rsvmap_off + rsvmap_len;
        let strings_off = struct_off + self.structure.len();
        let total_size = strings_off + self.strings.len();

        let mut blob = Vec::with_capacity(total_size);
        for word in [
            FDT_MAGIC,
            total_size as u32,
            struct_off as u32,
            strings_off as u32,
            rsvmap_off as u32,
            FDT_VERSION,
            FDT_LAST_COMP_VERSION,
            0, // boot_cpuid_phys
            self.strings.len() as u32,
            self.structure.len() as u32,
        ] {
            blob.extend_from_slice(&word.to_be_bytes());
        }
        blob.extend_from_slice(&[0u8; 16]); // empty reservation map
        blob.extend_from_slice(&self.structure);
        blob.extend_from_slice(&self.strings);
        blob
    }
}

impl Default for FdtBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_tree_layout() {
        let mut fdt = FdtBuilder::new();
        fdt.begin_node("");
        fdt.prop_str("compatible", "test");
        fdt.prop_u32("#address-cells", 2);
        fdt.begin_node("memory@0");
        fdt.prop_str("device_type", "memory");
        fdt.end_node();
        fdt.end_node();
        let blob = fdt.finish();

        // Header sanity.
        let magic = u32::from_be_bytes(blob[0..4].try_into().unwrap());
        let total = u32::from_be_bytes(blob[4..8].try_into().unwrap());
        let struct_off = u32::from_be_bytes(blob[8..12].try_into().unwrap()) as usize;
        let strings_off = u32::from_be_bytes(blob[12..16].try_into().unwrap()) as usize;
        let version = u32::from_be_bytes(blob[20..24].try_into().unwrap());
        assert_eq!(magic, 0xD00D_FEED);
        assert_eq!(total as usize, blob.len());
        assert_eq!(version, 17);
        assert!(struct_off < strings_off && strings_off < blob.len());

        // Structure starts with BEGIN_NODE of the root.
        let first = u32::from_be_bytes(blob[struct_off..struct_off + 4].try_into().unwrap());
        assert_eq!(first, 0x1);
        // Last token is FDT_END.
        let end = u32::from_be_bytes(
            blob[strings_off - 4..strings_off].try_into().unwrap(),
        );
        assert_eq!(end, 0x9);
        // Strings table contains deduplicated property names.
        let strings = &blob[strings_off..];
        assert!(strings.windows(11).any(|w| w == b"compatible\0"));
    }

    #[test]
    fn alignment_of_props() {
        let mut fdt = FdtBuilder::new();
        fdt.begin_node("");
        fdt.prop_str("bootargs", "abc"); // 4 bytes incl NUL -> aligned
        fdt.prop_str("stdout-path", "abcd"); // 5 bytes -> padded to 8
        fdt.end_node();
        let blob = fdt.finish();
        assert_eq!(blob.len() % 4, 0);
    }
}
