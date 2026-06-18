SHELL := /bin/bash

.PHONY: default clean realclean build-parser download extract parse all test pipeline

default: all

VERSION = 00bet9
XMLDIR = xml
ASLDIR = asl
PARSEDIR = asl-parsed

ASL_PARSER_RS := $(abspath ./asl-parser-rs)
ASL_PARSER_BIN := $(ASL_PARSER_RS)/target/release/asl-parser

# =============================================================================
# Build the Rust parser (with XML extraction support)
# =============================================================================

${ASL_PARSER_BIN}:
	cd ${ASL_PARSER_RS} && cargo build --release --features serde,xml-extract

build-parser: ${ASL_PARSER_BIN}

# =============================================================================
# Download XML files using Rust tool
# =============================================================================

.PHONY: download
download: ${ASL_PARSER_BIN} | ${XMLDIR}
	${ASL_PARSER_BIN} download --output ${XMLDIR} --version ${VERSION}

${XMLDIR}:
	mkdir -p ${XMLDIR}

${ASLDIR}:
	mkdir -p ${ASLDIR}

${PARSEDIR}:
	mkdir -p ${PARSEDIR}

# =============================================================================
# Extract ASL from XML using Rust tool
# =============================================================================

.PHONY: extract
extract: ${ASL_PARSER_BIN} download | ${ASLDIR}
	${ASL_PARSER_BIN} extract --input ${XMLDIR} --output ${ASLDIR} --version ${VERSION} --demangle

ASL_FILES = ${ASLDIR}/arm_defs.asl ${ASLDIR}/arm_instrs.asl ${ASLDIR}/arm_regs.asl

# Create empty support.asl placeholder
${ASLDIR}/support.asl: | ${ASLDIR}
	touch $@

# =============================================================================
# Parse ASL files with Rust parser
# =============================================================================

# Note: parse targets depend on extract having run first
${PARSEDIR}/arm_defs.json: ${ASLDIR}/arm_defs.asl ${ASL_PARSER_BIN} | ${PARSEDIR}
	${ASL_PARSER_BIN} definitions --format json $< > $@ 2>&1 || (cat $@ && rm -f $@ && exit 1)

${PARSEDIR}/arm_instrs.json: ${ASLDIR}/arm_instrs.asl ${ASL_PARSER_BIN} | ${PARSEDIR}
	${ASL_PARSER_BIN} instructions --format json $< > $@ 2>&1 || (cat $@ && rm -f $@ && exit 1)

${PARSEDIR}/arm_regs.json: ${ASLDIR}/arm_regs.asl ${ASL_PARSER_BIN} | ${PARSEDIR}
	${ASL_PARSER_BIN} registers --format json $< > $@ 2>&1 || (cat $@ && rm -f $@ && exit 1)

${PARSEDIR}/support.json: ${ASLDIR}/support.asl ${ASL_PARSER_BIN} | ${PARSEDIR}
	${ASL_PARSER_BIN} definitions --format json $< > $@ 2>&1 || (cat $@ && rm -f $@ && exit 1)

PARSED_FILES = ${PARSEDIR}/arm_defs.json ${PARSEDIR}/arm_instrs.json ${PARSEDIR}/arm_regs.json ${PARSEDIR}/support.json

parse: extract ${ASLDIR}/support.asl ${PARSED_FILES}

# =============================================================================
# Full pipeline using single Rust command
# =============================================================================

pipeline: ${ASL_PARSER_BIN}
	${ASL_PARSER_BIN} pipeline --workdir . --version ${VERSION} --demangle

# =============================================================================
# Quick test - download, extract and show summary
# =============================================================================

test: ${ASL_PARSER_BIN}
	@echo "=== Running full Rust pipeline ==="
	${ASL_PARSER_BIN} pipeline --workdir . --version ${VERSION} --demangle

# =============================================================================
# Main targets
# =============================================================================

all: build-parser extract ${ASLDIR}/support.asl parse

# =============================================================================
# Cleanup
# =============================================================================

clean:
	rm -rf ${ASLDIR} ${PARSEDIR}

realclean: clean
	rm -rf ${XMLDIR}
	cd ${ASL_PARSER_RS} && cargo clean

# =============================================================================
# Help
# =============================================================================

help:
	@echo "ARM ASL Parser - Pure Rust Implementation"
	@echo ""
	@echo "Targets:"
	@echo "  all          - Full pipeline: download, extract, and parse to JSON"
	@echo "  build-parser - Build the Rust parser with XML extraction support"
	@echo "  download     - Download ARM XML specification files"
	@echo "  extract      - Extract ASL from downloaded XML files"
	@echo "  parse        - Parse extracted ASL files to JSON"
	@echo "  pipeline     - Run complete pipeline with single Rust command"
	@echo "  test         - Run the full pipeline and show summary"
	@echo "  clean        - Remove extracted ASL and parsed files"
	@echo "  realclean    - Remove all generated files including XML"
	@echo ""
	@echo "The entire pipeline is now implemented in Rust:"
	@echo "  1. Download: Fetches ARM XML specs from developer.arm.com"
	@echo "  2. Extract:  Parses XML to generate ASL source files"
	@echo "  3. Parse:    Parses ASL to produce structured JSON output"
