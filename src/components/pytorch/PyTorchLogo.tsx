import React from 'react';

/** PyTorch flame mark — brand color #EE4C2C (see pytorch.org docs). */
const PyTorchLogo: React.FC<{ size?: number; className?: string }> = ({ size = 28, className }) => (
    <svg
        width={size}
        height={size}
        viewBox="0 0 32 32"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        className={className}
        aria-label="PyTorch"
        role="img"
    >
        <circle cx="16" cy="16" r="15" fill="#EE4C2C" opacity="0.15" />
        <path
            d="M16 4c-1.2 3.8-4.2 6.8-8 8 3.8 1.2 6.8 4.2 8 8 1.2-3.8 4.2-6.8 8-8-3.8-1.2-6.8-4.2-8-8Z"
            fill="#EE4C2C"
        />
        <circle cx="22.5" cy="9.5" r="2.2" fill="#F89C1C" stroke="#EE4C2C" strokeWidth="0.6" />
    </svg>
);

export default PyTorchLogo;
