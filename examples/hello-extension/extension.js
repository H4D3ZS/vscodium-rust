// Minimal HADES extension: one command + one status bar item.
// API contract: packages/hades-extension-api/index.d.ts
const hades = require('hades');

exports.activate = () => {
    const cmd = hades.commands.register('hello.sayHi', () => {
        hades.window.showMessage('Hi from hello-extension!');
    });

    const item = hades.window.createStatusBarItem('right', 10);
    item.text = '$(rocket) Hello';
    item.tooltip = 'hello-extension is alive';
    item.show();

    return {
        dispose() {
            cmd.dispose();
            item.dispose();
        },
    };
};
