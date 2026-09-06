import React, { useState } from 'react';
import {
    Modal, ModalContent, ModalHeader, ModalBody, ModalFooter,
    Input, Button, Accordion, AccordionItem,
} from '@heroui/react';
import { IconUser, IconAlertCircle } from '@tabler/icons-react';
import { useStore } from '../../store';

// ─────────────────────────────────────────────────────────────────────────────
//  LoginModal — HeroUI pilot. Sign in to unlock cloud models. Local models
//  (Lemonade/the local backend) never need this; the modal only appears when opened from
//  the Account panel or when a cloud model is used while signed out.
// ─────────────────────────────────────────────────────────────────────────────

const LoginModal: React.FC = () => {
    const open = useStore(s => s.isLoginModalOpen);
    const close = useStore(s => s.closeLoginModal);
    const login = useStore(s => s.login);
    const status = useStore(s => s.authStatus);
    const error = useStore(s => s.authError);
    const authUrl = useStore(s => s.authUrl);
    const setAuthUrl = useStore(s => s.setAuthUrl);

    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const busy = status === 'authenticating';

    const submit = async () => {
        if (!email.trim() || busy) return;
        await login(email.trim(), password);
    };

    return (
        <Modal
            isOpen={open}
            onOpenChange={(o) => { if (!o && !busy) close(); }}
            placement="center"
            size="md"
            backdrop="blur"
            isDismissable={!busy}
        >
            <ModalContent>
                <ModalHeader className="flex items-center gap-2">
                    <IconUser size={18} className="text-primary" />
                    Sign in to unlock cloud models
                </ModalHeader>
                <ModalBody>
                    <p className="text-small text-default-500 -mt-1">
                        Local models (Lemonade) stay free and need no account. Signing in
                        unlocks powerful cloud models like <b>GLM-5.2</b> and <b>Qwen3.6-35B-MoE</b>.
                    </p>

                    <Input
                        label="Email"
                        type="email"
                        value={email}
                        onValueChange={setEmail}
                        placeholder="you@example.com"
                        variant="bordered"
                        autoFocus
                        isDisabled={busy}
                        onKeyDown={(e) => { if (e.key === 'Enter') submit(); }}
                    />
                    <Input
                        label="Password"
                        type="password"
                        value={password}
                        onValueChange={setPassword}
                        placeholder="••••••••"
                        variant="bordered"
                        isDisabled={busy}
                        onKeyDown={(e) => { if (e.key === 'Enter') submit(); }}
                    />

                    <Accordion isCompact className="px-0">
                        <AccordionItem
                            key="server"
                            aria-label="Auth server"
                            title={<span className="text-tiny text-default-500">Auth server</span>}
                        >
                            <Input
                                size="sm"
                                value={authUrl}
                                onValueChange={setAuthUrl}
                                placeholder="https://api.cyberifrit.xyz/auth"
                                variant="bordered"
                                isDisabled={busy}
                            />
                            <p className="text-tiny text-default-400 mt-1">
                                Dev: set <code>localStorage.subscription.mockAuth=true</code> to test without a server.
                            </p>
                        </AccordionItem>
                    </Accordion>

                    {error && (
                        <div className="flex items-center gap-2 text-small text-danger">
                            <IconAlertCircle size={16} /> {error}
                        </div>
                    )}
                </ModalBody>
                <ModalFooter>
                    <Button variant="bordered" onPress={close} isDisabled={busy}>Cancel</Button>
                    <Button color="primary" onPress={submit} isLoading={busy} isDisabled={!email.trim()}>
                        {busy ? 'Signing in…' : 'Sign in'}
                    </Button>
                </ModalFooter>
            </ModalContent>
        </Modal>
    );
};

export default LoginModal;
