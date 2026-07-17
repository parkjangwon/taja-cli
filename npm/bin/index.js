#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const { spawn } = require('child_process');
const https = require('https');

// 버전 정보 (Github Releases 태그와 매핑)
const VERSION = '0.1.1';
const REPO = 'parkjangwon/taja-cli';

// 플랫폼 매핑
const PLATFORMS = {
  'darwin-arm64': 'taja-cli-macos-arm64.tar.gz',
  'darwin-x64': 'taja-cli-macos-x64.tar.gz',
  'linux-x64': 'taja-cli-linux-x64.tar.gz',
  'linux-arm64': 'taja-cli-linux-arm64.tar.gz',
  'win32-x64': 'taja-cli-windows-x64.zip'
};

const key = `${process.platform}-${process.arch}`;
const archiveName = PLATFORMS[key];

if (!archiveName) {
  console.error(`Error: Unsupported platform/architecture: ${key}`);
  process.exit(1);
}

const binDir = path.join(__dirname, '..', 'dist');
const binName = process.platform === 'win32' ? 'taja-cli.exe' : 'taja-cli';
const binPath = path.join(binDir, binName);

function downloadAndExtract() {
  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }

  const url = `https://github.com/${REPO}/releases/download/v${VERSION}/${archiveName}`;
  console.log(`Downloading taja-cli binary from ${url}...`);

  return new Promise((resolve, reject) => {
    function getStream(targetUrl) {
      https.get(targetUrl, (res) => {
        if (res.statusCode === 302 || res.statusCode === 301) {
          getStream(res.headers.location);
        } else if (res.statusCode === 200) {
          const tempTarPath = path.join(binDir, 'temp.tar.gz');
          const file = fs.createWriteStream(tempTarPath);
          res.pipe(file);
          
          file.on('finish', () => {
            file.close();
            try {
              const tarCmd = process.platform === 'win32'
                ? `tar -xf "${tempTarPath}" -C "${binDir}"`
                : `tar -xzf "${tempTarPath}" -C "${binDir}"`;
                
              require('child_process').exec(tarCmd, (err) => {
                // 임시 파일 삭제
                if (fs.existsSync(tempTarPath)) {
                  fs.unlinkSync(tempTarPath);
                }
                if (err) {
                  reject(err);
                } else {
                  if (process.platform !== 'win32') {
                    fs.chmodSync(binPath, 0o755);
                  }
                  resolve();
                }
              });
            } catch (e) {
              reject(e);
            }
          });
        } else {
          reject(new Error(`Failed to download binary: HTTP ${res.statusCode}`));
        }
      }).on('error', reject);
    }
    getStream(url);
  });
}

function runBinary() {
  const args = process.argv.slice(2);
  const child = spawn(binPath, args, { stdio: 'inherit' });
  child.on('close', (code) => {
    process.exit(code);
  });
  child.on('error', (err) => {
    console.error('Failed to start taja-cli binary:', err);
    process.exit(1);
  });
}

// 메인 루틴
if (fs.existsSync(binPath)) {
  runBinary();
} else {
  downloadAndExtract()
    .then(() => {
      runBinary();
    })
    .catch((err) => {
      console.error('Failed to install taja-cli:', err);
      process.exit(1);
    });
}
