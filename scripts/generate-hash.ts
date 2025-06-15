import * as bcrypt from 'bcryptjs';

async function generateHash() {
  const password = 'R5hub2025flowlight';
  const salt = await bcrypt.genSalt(10);
  const hash = await bcrypt.hash(password, salt);
  
  console.log('Senha original:', password);
  console.log('Hash gerado:', hash);
  
  // Verificação
  const isMatch = await bcrypt.compare(password, hash);
  console.log('Verificação:', isMatch ? 'Senha válida' : 'Senha inválida');
}

generateHash().catch(console.error);
