@echo off
"%~dp0..\typescript-language-server\node-v20.18.0-win-x64\node.exe" "%~dp0node_modules\pyright\langserver.index.js" %*
