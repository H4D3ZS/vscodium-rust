import React, { useRef } from 'react';
import { useStore } from '../../store';

/**
 * Image attachment button — pick one or more image files and attach them to the
 * composer as vision context (base64 data URL, with a thumbnail chip). Replaces the
 * old screen-capture button. Self-contained: writes straight to the attachedFiles
 * store, the same shape the drag-and-drop image path uses.
 */
const ScreenRecordingButton: React.FC<{ onAttached?: (path: string) => void }> = ({ onAttached }) => {
    const attachFile = useStore(s => s.attachFile);
    const inputRef = useRef<HTMLInputElement | null>(null);

    const readAsDataUrl = (file: File): Promise<string> =>
        new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.onload = () => resolve(reader.result as string);
            reader.onerror = reject;
            reader.readAsDataURL(file);
        });

    const onPick = async (e: React.ChangeEvent<HTMLInputElement>) => {
        const files = Array.from(e.target.files || []);
        for (const file of files) {
            if (!file.type.startsWith('image/')) continue;
            try {
                const dataUrl = await readAsDataUrl(file);
                const path = (file as any).path || `${file.name}-${Date.now()}`;
                attachFile({
                    id: `img-${Date.now()}-${file.name}`,
                    type: 'attachment',
                    name: file.name,
                    path,
                    data: dataUrl,
                    thumbnail: dataUrl,
                } as any);
                onAttached?.(path);
            } catch (err) {
                console.warn('[image-attach] failed:', err);
            }
        }
        // Reset so picking the same file again still fires onChange.
        if (inputRef.current) inputRef.current.value = '';
    };

    return (
        <>
            <input
                ref={inputRef}
                type="file"
                accept="image/*"
                multiple
                style={{ display: 'none' }}
                onChange={(e) => void onPick(e)}
            />
            <button
                type="button"
                title="Attach image (for vision models)"
                onClick={() => inputRef.current?.click()}
                style={{
                    border: 'none',
                    background: 'transparent',
                    color: 'rgba(255,255,255,0.6)',
                    cursor: 'pointer',
                    padding: '4px 6px',
                    borderRadius: 4,
                }}
            >
                <i className="codicon codicon-file-media" />
            </button>
        </>
    );
};

export default ScreenRecordingButton;
