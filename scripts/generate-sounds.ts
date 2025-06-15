// Este script gere sons de notificação básicos usando o Web Audio API
// Para executar: ts-node scripts/generate-sounds.ts

import * as fs from 'fs';
import * as path from 'path';
import { execSync } from 'child_process';

const SOUNDS_DIR = path.join(__dirname, '../public/sounds');

// Criar diretório de sons se não existir
if (!fs.existsSync(SOUNDS_DIR)) {
  fs.mkdirSync(SOUNDS_DIR, { recursive: true });
}

// Configurações dos sons
const SOUNDS = {
  info: { type: 'sine', frequency: 440, duration: 0.2 },
  success: { type: 'sine', frequency: 880, duration: 0.3 },
  warning: { type: 'sine', frequency: 660, duration: 0.4 },
  error: { type: 'sine', frequency: 330, duration: 0.5 },
  update: { type: 'sine', frequency: 523.25, duration: 0.3 }, // Nota C5
  reminder: { type: 'sine', frequency: 784, duration: 0.2 } // Nota G5
};

// Gerar arquivos de som usando o SoX (Sound eXchange)
// Instale o SoX: https://sox.sourceforge.net/
function generateSoundFile(type: string, frequency: number, duration: number) {
  const outputFile = path.join(SOUNDS_DIR, `${type}.mp3`);
  
  try {
    // Usando o SoX para gerar o som
    execSync(`sox -n -r 44100 -c 2 ${outputFile} synth ${duration} sine ${frequency} vol 0.5`);
    console.log(`✅ Som gerado: ${outputFile}`);
  } catch (error) {
    console.error(`❌ Erro ao gerar som ${type}:`, error.message);
    console.log('Certifique-se de que o SoX (Sound eXchange) está instalado.');
    console.log('Instale com: brew install sox (macOS) ou apt-get install sox (Linux)');
  }
}

// Gerar todos os sons
Object.entries(SOUNDS).forEach(([type, { frequency, duration }]) => {
  generateSoundFile(type, frequency, duration);
});

console.log('\n🎉 Sons de notificação gerados com sucesso!');
