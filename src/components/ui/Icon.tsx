import React from 'react';
import {
    IconRobot, IconBolt, IconBrain, IconCloud, IconCloudLock, IconDeviceDesktop,
    IconCheck, IconX, IconAlertTriangle, IconInfoCircle, IconLoader2,
    IconMicrophone, IconVolume, IconEye, IconCode, IconHierarchy, IconSparkles,
    IconRocket, IconShieldLock, IconTerminal2, IconFileText, IconSearch,
    IconUser, IconLock, IconPlugConnected, IconChartDots3, IconWand,
    // Tool-feed icons
    IconFilePencil, IconReplace, IconDeviceFloppy, IconPencil, IconFlask,
    IconGhost2, IconFolder, IconFolderPlus, IconGitCommit, IconGitCompare,
    IconBox, IconWorld, IconKey, IconBomb, IconSettings, IconZoomCode,
    type IconProps,
} from '@tabler/icons-react';

// ─────────────────────────────────────────────────────────────────────────────
//  Icon — single entry point for Tabler icons across the IDE.
//
//  Replaces ad-hoc emojis with consistent, professional vector icons. Codicons
//  (the editor-native set) stay where they are; this is for everything that used
//  to be an emoji. Use <Icon name="bolt" /> or import a Tabler icon directly.
//
//  Default size 16 / stroke 1.75 matches the IDE's text-scale UI.
// ─────────────────────────────────────────────────────────────────────────────

const REGISTRY = {
    agent: IconRobot,
    bolt: IconBolt,
    brain: IconBrain,
    cloud: IconCloud,
    'cloud-lock': IconCloudLock,
    desktop: IconDeviceDesktop,
    check: IconCheck,
    x: IconX,
    warning: IconAlertTriangle,
    info: IconInfoCircle,
    spinner: IconLoader2,
    mic: IconMicrophone,
    volume: IconVolume,
    eye: IconEye,
    code: IconCode,
    diagram: IconHierarchy,
    sparkles: IconSparkles,
    rocket: IconRocket,
    shield: IconShieldLock,
    terminal: IconTerminal2,
    file: IconFileText,
    search: IconSearch,
    user: IconUser,
    lock: IconLock,
    plug: IconPlugConnected,
    graph: IconChartDots3,
    wand: IconWand,
} as const;

export type IconName = keyof typeof REGISTRY;

export interface UiIconProps extends Omit<IconProps, 'ref'> {
    name: IconName;
}

/** Render a registered Tabler icon by name with IDE-consistent defaults. */
export const Icon: React.FC<UiIconProps> = ({ name, size = 16, stroke = 1.75, ...rest }) => {
    const Cmp = REGISTRY[name];
    if (!Cmp) return null;
    return <Cmp size={size} stroke={stroke} {...rest} />;
};

// Agent tool-call → icon, replacing the old emoji map. Keyed by backend tool name.
const TOOL_ICONS: Record<string, React.ComponentType<IconProps>> = {
    write_to_file: IconFilePencil,
    search_replace_edit: IconReplace,
    str_replace: IconReplace,
    apply_shadow_patch: IconDeviceFloppy,
    patch_file_content: IconPencil,
    view_file: IconEye,
    run_command: IconTerminal2,
    verify_implementation: IconFlask,
    ghost_test: IconGhost2,
    list_files: IconFolder,
    grep: IconSearch,
    git_commit: IconGitCommit,
    dev_cargo_diagnostics: IconBox,
    web_search: IconWorld,
    git_diff: IconGitCompare,
    semantic_search: IconBrain,
    find_symbols: IconZoomCode,
    create_directory: IconFolderPlus,
    deep_security_audit: IconShieldLock,
    secrets_scan: IconKey,
    weaponize_env: IconBomb,
};

/** Icon for an agent tool-call name (falls back to a gear). */
export const ToolIcon: React.FC<{ tool: string } & Omit<IconProps, 'ref'>> = ({ tool, size = 13, stroke = 1.75, ...rest }) => {
    const Cmp = TOOL_ICONS[tool] || IconSettings;
    return <Cmp size={size} stroke={stroke} {...rest} />;
};

export default Icon;
