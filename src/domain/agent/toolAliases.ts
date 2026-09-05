/**
 * Frontend mirror of `src-tauri/src/tool_aliases.rs`.
 * Keep in sync when adding aliases — one canonical name per model variant.
 */

const ALIAS_MAP: Record<string, string> = {
    // Terminal
    bash: 'run_command', sh: 'run_command', shell: 'run_command', exec: 'run_command',
    execute: 'run_command', cmd: 'run_command', run: 'run_command', terminal: 'run_command',
    run_terminal_cmd: 'run_command', run_terminal_command: 'run_command',
    execute_command: 'run_command', execute_bash: 'run_command', shell_command: 'run_command',
    run_shell_command: 'run_command', terminal_command: 'run_command',
    // Read / write
    file_read: 'view_file', read_file: 'view_file', cat: 'view_file', read: 'view_file',
    view: 'view_file', get_file: 'view_file',
    file_write: 'write_to_file', write_file: 'write_to_file', create_file: 'write_to_file',
    save_file: 'write_to_file', write: 'write_to_file',
    // Edit
    str_replace: 'str_replace', string_replace: 'str_replace', replace_in_file: 'str_replace',
    file_edit: 'search_replace_edit', edit_file: 'search_replace_edit', edit: 'search_replace_edit',
    code_edit: 'search_replace_edit', search_replace: 'search_replace_edit',
    str_replace_editor: 'search_replace_edit', replace: 'search_replace_edit',
    patch: 'patch_file_content', patch_lines: 'patch_file_content', line_edit: 'patch_file_content',
    // FS
    glob: 'find_by_name', find: 'find_by_name', find_files: 'find_by_name',
    glob_file_search: 'find_by_name', file_glob: 'find_by_name', glob_files: 'find_by_name',
    list_directory: 'list_files', list_dir: 'list_files', ls: 'list_files', dir: 'list_files',
    mkdir: 'create_directory', create_dir: 'create_directory',
    // Search
    grep: 'grep', ripgrep: 'grep', grep_search: 'grep', find_string: 'grep',
    find_in_files: 'grep', search: 'grep', code_search: 'grep',
    codebase_search: 'search_codebase', codebaseSearch: 'search_codebase',
    semantic_search: 'semantic_search', search_index: 'semantic_search',
    // Web
    web_fetch: 'web_fetch', read_url_content: 'web_fetch', fetch_url: 'web_fetch',
    internet_search: 'web_search', research: 'web_search', browse: 'web_search',
    crawl: 'crawl_url', scrape: 'crawl_url', extract_page: 'crawl_url',
    deep_scrape: 'deep_crawl', crawl_site: 'deep_crawl', site_map: 'deep_crawl',
    // Pentest
    nmap: 'network_port_scanner', port_scan: 'network_port_scanner', scan_ports: 'network_port_scanner',
    nmap_scan: 'network_scan', host_scan: 'network_scan', network_recon: 'network_scan',
    searchsploit: 'exploit_lookup', exploit_search: 'exploit_lookup', cve_lookup: 'exploit_lookup',
    live_attack: 'apex_simulate_attack', apex_execute_attack: 'apex_simulate_attack',
    attack_simulation: 'apex_simulate_attack',
    vuln_hunt: 'ai_vuln_hunt', bug_hunt: 'ai_vuln_hunt', vuln_scan: 'deep_security_audit',
    security_audit: 'deep_security_audit', web_audit: 'web_security_audit', url_audit: 'web_security_audit',
    red_team: 'apex_red_team_scan', pentest_scan: 'apex_red_team_scan',
    reverse_shell: 'reverse_shell_generate', rev_shell: 'reverse_shell_generate',
    weaponize: 'weaponize_env', kali_tools: 'sec_distro_inventory', sec_distro: 'sec_distro_inventory',
    find_secrets: 'secrets_scan', secret_scan: 'secrets_scan',
    // Dev
    cargo_check: 'dev_cargo_diagnostics', diagnostics: 'dev_cargo_diagnostics', check: 'dev_cargo_diagnostics',
    verify: 'verify_implementation', build: 'verify_implementation', test: 'verify_implementation',
    skill_execute: 'use_skill', run_skill: 'use_skill',
};

/** TS registry name when backend canonical differs (for getToolByName fallback). */
export const BACKEND_TO_TS_TOOL: Record<string, string> = {
    run_command: 'bash',
    view_file: 'file_read',
    write_to_file: 'file_write',
    find_by_name: 'glob',
    grep: 'grep',
    list_files: 'list_directory',
    web_fetch: 'web_fetch',
    web_search: 'web_search',
    crawl_url: 'crawl_url',
    deep_crawl: 'deep_crawl',
};

export function canonicalToolName(name: string): string {
    const raw = (name || '').trim();
    const key = raw.replace(/-/g, '_').toLowerCase();
    return ALIAS_MAP[key] || raw;
}

export function toolsMatchForFinish(a: string, b: string, callIdA?: string, callIdB?: string): boolean {
    if (callIdA && callIdB && callIdA === callIdB) return true;
    if (a === b) return true;
    return canonicalToolName(a) === canonicalToolName(b);
}
