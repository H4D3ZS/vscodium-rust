import idleImg from '../../assets/sentient_avatar/idle.png';
import thinkingImg from '../../assets/sentient_avatar/thinking.png';
import codingImg from '../../assets/sentient_avatar/coding.png';
import errorImg from '../../assets/sentient_avatar/error.png';

export type AvatarState = 'idle' | 'thinking' | 'coding' | 'error';

interface SentientAvatarProps {
    state: AvatarState;
    size?: number;
}

const SentientAvatar: React.FC<SentientAvatarProps> = ({ state, size = 32 }) => {
    const assets = {
        idle: idleImg,
        thinking: thinkingImg,
        coding: codingImg,
        error: errorImg
    };

    const getAnimation = () => {
        switch (state) {
            case 'idle': return 'float 3s ease-in-out infinite';
            case 'thinking': return 'pulse 1s ease-in-out infinite';
            case 'coding': return 'spin-slow 10s linear infinite';
            case 'error': return 'shake 0.5s linear infinite';
            default: return 'none';
        }
    };

    return (
        <div style={{
            width: size,
            height: size,
            borderRadius: '50%',
            overflow: 'hidden',
            flexShrink: 0,
            boxShadow: `0 0 15px ${state === 'error' ? 'rgba(239, 68, 68, 0.4)' : state === 'thinking' ? 'rgba(167, 139, 250, 0.4)' : 'rgba(59, 130, 246, 0.2)'}`,
            border: `1px solid ${state === 'error' ? '#ef4444' : 'rgba(255,255,255,0.1)'}`,
            position: 'relative',
            background: '#000',
            animation: getAnimation()
        }}>
            <style>{`
                @keyframes float {
                    0%, 100% { transform: translateY(0); }
                    50% { transform: translateY(-3px); }
                }
                @keyframes pulse {
                    0%, 100% { transform: scale(1); opacity: 0.8; }
                    50% { transform: scale(1.05); opacity: 1; }
                }
                @keyframes spin-slow {
                    from { transform: rotate(0deg); }
                    to { transform: rotate(360deg); }
                }
                @keyframes shake {
                    0%, 100% { transform: translateX(0); }
                    25% { transform: translateX(-2px); }
                    75% { transform: translateX(2px); }
                }
            `}</style>
            <img
                src={assets[state]}
                alt={`AI State: ${state}`}
                style={{
                    width: '100%',
                    height: '100%',
                    objectFit: 'cover',
                    transition: 'all 0.5s ease-in-out',
                    transform: state === 'thinking' ? 'scale(1.1)' : 'scale(1)'
                }}
            />
        </div>
    );
};

export default SentientAvatar;
