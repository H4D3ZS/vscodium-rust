import React, { useState } from 'react';
import {
    Modal, TextField, Label, Input, Button, Accordion,
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
        <Modal>
            <Modal.Backdrop
                variant="blur"
                isOpen={open}
                onOpenChange={(o) => { if (!o && !busy) close(); }}
                isDismissable={!busy}
            >
                <Modal.Container placement="center" size="md">
                    <Modal.Dialog>
                        <Modal.CloseTrigger />
                        <Modal.Header>
                            <Modal.Icon><IconUser size={18} className="text-accent" /></Modal.Icon>
                            <Modal.Heading>Sign in to unlock cloud models</Modal.Heading>
                        </Modal.Header>
                        <Modal.Body>
                            <p className="text-sm text-muted -mt-1">
                                Local models (Lemonade) stay free and need no account. Signing in
                                unlocks powerful cloud models like <b>GLM-5.2</b> and <b>Qwen3.6-35B-MoE</b>.
                            </p>

                            <TextField name="email" type="email" isDisabled={busy}>
                                <Label>Email</Label>
                                <Input
                                    value={email}
                                    onChange={(e) => setEmail(e.target.value)}
                                    placeholder="you@example.com"
                                    variant="secondary"
                                    autoFocus
                                    onKeyDown={(e) => { if (e.key === 'Enter') submit(); }}
                                />
                            </TextField>
                            <TextField name="password" type="password" isDisabled={busy}>
                                <Label>Password</Label>
                                <Input
                                    value={password}
                                    onChange={(e) => setPassword(e.target.value)}
                                    placeholder="••••••••"
                                    variant="secondary"
                                    onKeyDown={(e) => { if (e.key === 'Enter') submit(); }}
                                />
                            </TextField>

                            <Accordion className="px-0">
                                <Accordion.Item id="server">
                                    <Accordion.Heading>
                                        <Accordion.Trigger aria-label="Auth server">
                                            <span className="text-xs text-muted">Auth server</span>
                                            <Accordion.Indicator />
                                        </Accordion.Trigger>
                                    </Accordion.Heading>
                                    <Accordion.Panel>
                                        <Accordion.Body>
                                            <Input
                                                value={authUrl}
                                                onChange={(e) => setAuthUrl(e.target.value)}
                                                placeholder="https://api.cyberifrit.xyz/auth"
                                                variant="secondary"
                                                disabled={busy}
                                            />
                                            <p className="text-xs text-muted mt-1">
                                                Dev: set <code>localStorage.subscription.mockAuth=true</code> to test without a server.
                                            </p>
                                        </Accordion.Body>
                                    </Accordion.Panel>
                                </Accordion.Item>
                            </Accordion>

                            {error && (
                                <div className="flex items-center gap-2 text-sm text-danger">
                                    <IconAlertCircle size={16} /> {error}
                                </div>
                            )}
                        </Modal.Body>
                        <Modal.Footer>
                            <Button variant="secondary" onPress={close} isDisabled={busy}>Cancel</Button>
                            <Button variant="primary" onPress={submit} isPending={busy} isDisabled={!email.trim()}>
                                {busy ? 'Signing in…' : 'Sign in'}
                            </Button>
                        </Modal.Footer>
                    </Modal.Dialog>
                </Modal.Container>
            </Modal.Backdrop>
        </Modal>
    );
};

export default LoginModal;
