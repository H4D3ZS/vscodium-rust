import React, { useEffect, useState, useRef } from 'react';
import { useStore } from '../store';
import { digitalLife } from '../digital-life';

/**
 * AIRI Conversation Panel
 * 
 * Simple voice/text chat interface - separate from full VRM mode
 * Shows in AI Agent panel, always accessible
 */

const AiriConversation: React.FC = () => {
  const [isListening, setIsListening] = useState(false);
  const [messages, setMessages] = useState<Array<{
    role: 'user' | 'airi';
    text: string;
    timestamp: number;
    emotion?: string;
  }>>([]);
  const [inputText, setInputText] = useState('');
  const [showChat, setShowChat] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Initialize Digital Life
  useEffect(() => {
    
    // Load conversation history
    const saved = localStorage.getItem('airi_conversations');
    if (saved) {
      const history = JSON.parse(saved);
      setMessages(history.flatMap((h: any) => [
        {
          role: 'user' as const,
          text: h.user,
          timestamp: h.timestamp,
        },
        {
          role: 'airi' as const,
          text: h.airi,
          timestamp: h.timestamp,
          emotion: h.emotion,
        },
      ]));
    }

    // Listen for AIRI ambient speech
    const handleAiriSpeech = (e: any) => {
      if (e.detail?.text) {
        addMessage('airi', e.detail.text, e.detail.emotion);
      }
    };

    window.addEventListener('airi-speech', handleAiriSpeech);
    return () => window.removeEventListener('airi-speech', handleAiriSpeech);
  }, []);

  // Auto-scroll to bottom
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const addMessage = (role: 'user' | 'airi', text: string, emotion?: string) => {
    setMessages(prev => [...prev, {
      role,
      text,
      timestamp: Date.now(),
      emotion,
    }]);
  };

  const handleSend = async () => {
    if (!inputText.trim()) return;

    // Add user message
    addMessage('user', inputText);
    
    // Save to conversation history
    await digitalLife.saveConversation(inputText, '', '');
    
    // Send to AIRI for response
    const response = await getAiriResponse(inputText);
    addMessage('airi', response, 'happy');
    
    // Save complete conversation
    await digitalLife.saveConversation(inputText, response, 'happy');
    
    setInputText('');
  };

  const getAiriResponse = async (userMessage: string): Promise<string> => {
    // Simple responses for now - integrate with actual AI later
    const responses = [
      "That's interesting! Tell me more.",
      "I'd be happy to help with that!",
      "Hmm, let me think about it...",
      "Great idea! What's the plan?",
      "I'm here for you! What do you need?",
    ];
    
    return responses[Math.floor(Math.random() * responses.length)];
  };

  const toggleListening = async () => {
    if (isListening) {
      setIsListening(false);
      // Stop listening
    } else {
      setIsListening(true);
      // Start voice recognition
      await startVoiceRecognition();
    }
  };

  const startVoiceRecognition = async () => {
    try {
      const { SpeechRecognition } = window as any;
      const recognition = new SpeechRecognition();
      
      recognition.onresult = (event: any) => {
        const transcript = event.results[0][0].transcript;
        setInputText(transcript);
        setIsListening(false);
      };

      recognition.onerror = () => {
        setIsListening(false);
      };

      recognition.start();
    } catch (e) {
      console.error('[Conversation] Voice recognition not supported');
      setIsListening(false);
    }
  };

  const toggleChat = () => {
    setShowChat(!showChat);
    digitalLife.configure({ showChat: !showChat });
  };

  return (
    <div style={{
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
      background: 'rgba(15, 23, 42, 0.5)',
    }}>
      {/* Header */}
      <div style={{
        padding: '12px',
        borderBottom: '1px solid rgba(255,255,255,0.1)',
        display: 'flex',
        justifyContent: 'space-between',
        alignItems: 'center',
      }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <div style={{
            width: '8px',
            height: '8px',
            borderRadius: '50%',
            background: '#10b981',
            animation: 'pulse 2s infinite',
          }}></div>
          <span style={{ fontSize: '12px', fontWeight: 600 }}>AIRI - Digital Life</span>
        </div>
        
        <button
          onClick={toggleChat}
          style={{
            background: 'transparent',
            border: 'none',
            color: 'var(--vscode-foreground)',
            cursor: 'pointer',
            padding: '4px',
            fontSize: '11px',
          }}
          title={showChat ? 'Hide chat' : 'Show chat'}
        >
          {showChat ? '💬 Visible' : '🔇 Voice Only'}
        </button>
      </div>

      {/* Messages */}
      {showChat && (
        <div style={{
          flex: 1,
          overflowY: 'auto',
          padding: '12px',
        }}>
          {messages.map((msg, i) => (
            <div
              key={i}
              style={{
                marginBottom: '12px',
                display: 'flex',
                flexDirection: msg.role === 'user' ? 'row-reverse' : 'row',
              }}
            >
              <div style={{
                maxWidth: '80%',
                padding: '8px 12px',
                borderRadius: '12px',
                background: msg.role === 'user'
                  ? 'var(--vscode-button-background)'
                  : 'rgba(255,255,255,0.1)',
                color: msg.role === 'user'
                  ? 'var(--vscode-button-foreground)'
                  : 'var(--vscode-foreground)',
                fontSize: '12px',
                lineHeight: 1.4,
              }}>
                {msg.text}
                <div style={{
                  fontSize: '9px',
                  opacity: 0.6,
                  marginTop: '4px',
                }}>
                  {new Date(msg.timestamp).toLocaleTimeString()}
                </div>
              </div>
            </div>
          ))}
          <div ref={messagesEndRef} />
        </div>
      )}

      {/* Input */}
      {showChat && (
        <div style={{
          padding: '12px',
          borderTop: '1px solid rgba(255,255,255,0.1)',
          display: 'flex',
          gap: '8px',
        }}>
          <input
            type="text"
            value={inputText}
            onChange={(e) => setInputText(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && handleSend()}
            placeholder="Talk to AIRI..."
            style={{
              flex: 1,
              background: 'var(--vscode-input-background)',
              color: 'var(--vscode-input-foreground)',
              border: '1px solid var(--vscode-input-border)',
              padding: '8px 12px',
              fontSize: '12px',
              borderRadius: '6px',
            }}
          />
          
          <button
            onClick={handleSend}
            style={{
              background: 'var(--vscode-button-background)',
              color: 'var(--vscode-button-foreground)',
              border: 'none',
              padding: '8px 16px',
              borderRadius: '6px',
              cursor: 'pointer',
              fontSize: '12px',
            }}
          >
            Send
          </button>
          
          <button
            onClick={toggleListening}
            style={{
              background: isListening ? '#ef4444' : 'var(--vscode-button-secondaryBackground)',
              color: 'var(--vscode-foreground)',
              border: 'none',
              padding: '8px 12px',
              borderRadius: '6px',
              cursor: 'pointer',
              fontSize: '16px',
            }}
            title={isListening ? 'Stop listening' : 'Start voice input'}
          >
            {isListening ? '🔴' : '🎤'}
          </button>
        </div>
      )}

      {/* Status Bar (always visible) */}
      <div style={{
        padding: '6px 12px',
        background: 'rgba(0,0,0,0.3)',
        fontSize: '10px',
        color: 'rgba(255,255,255,0.5)',
        display: 'flex',
        justifyContent: 'space-between',
      }}>
        <span>🟢 AIRI is {isListening ? 'listening...' : 'present'}</span>
        <span>{messages.length} conversations</span>
      </div>

      <style>{`
        @keyframes pulse {
          0%, 100% { opacity: 1; }
          50% { opacity: 0.5; }
        }
      `}</style>
    </div>
  );
};

export default AiriConversation;
