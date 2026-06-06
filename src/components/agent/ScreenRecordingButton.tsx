import React, { useRef, useState } from 'react';
import { useStore } from '../../store';
import { agBrainSaveMedia } from '../../infrastructure/antigravity/antigravityClient';

/** Capture display frame → `.agent/brain/{cascade}/media__*.png` (Antigravity visual artifact). */
const ScreenRecordingButton: React.FC<{ onAttached?: (path: string) => void }> = ({ onAttached }) => {
    const activeRoot = useStore(s => s.activeRoot);
    const activeCascadeId = useStore(s => s.activeCascadeId);
    const setActiveCascadeId = useStore(s => s.setActiveCascadeId);
    const [recording, setRecording] = useState(false);
    const streamRef = useRef<MediaStream | null>(null);
    const videoRef = useRef<HTMLVideoElement | null>(null);

    const stopAndCapture = async () => {
        const stream = streamRef.current;
        const video = videoRef.current;
        if (!stream || !video || !activeRoot) {
            setRecording(false);
            return;
        }
        const canvas = document.createElement('canvas');
        canvas.width = video.videoWidth || 1280;
        canvas.height = video.videoHeight || 720;
        const ctx = canvas.getContext('2d');
        if (ctx) ctx.drawImage(video, 0, 0, canvas.width, canvas.height);
        stream.getTracks().forEach(t => t.stop());
        streamRef.current = null;
        setRecording(false);

        const dataUrl = canvas.toDataURL('image/png');
        const b64 = dataUrl.split(',')[1];
        if (!b64) return;

        let cascadeId = activeCascadeId;
        if (!cascadeId) {
            cascadeId = `cascade-${Date.now()}`;
            setActiveCascadeId(cascadeId);
        }
        try {
            const path = await agBrainSaveMedia(activeRoot, cascadeId, b64, 'Screen capture');
            onAttached?.(path);
        } catch (e) {
            console.warn('[screen-capture] save failed:', e);
        }
    };

    const toggle = async () => {
        if (recording) {
            await stopAndCapture();
            return;
        }
        try {
            const stream = await navigator.mediaDevices.getDisplayMedia({ video: true, audio: false });
            streamRef.current = stream;
            const video = document.createElement('video');
            video.srcObject = stream;
            video.muted = true;
            await video.play();
            videoRef.current = video;
            setRecording(true);
        } catch (e) {
            console.warn('[screen-capture] denied:', e);
        }
    };

    return (
        <button
            type="button"
            title={recording ? 'Stop and save screenshot to brain' : 'Capture screen to mission artifacts'}
            onClick={() => void toggle()}
            style={{
                border: 'none',
                background: recording ? 'rgba(239,68,68,0.2)' : 'transparent',
                color: recording ? '#f87171' : 'rgba(255,255,255,0.6)',
                cursor: 'pointer',
                padding: '4px 6px',
                borderRadius: 4,
            }}
        >
            <i className={`codicon codicon-${recording ? 'debug-stop' : 'device-camera'}`} />
        </button>
    );
};

export default ScreenRecordingButton;
