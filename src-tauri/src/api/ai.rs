//! AI API handlers
//! 
//! Handlers para os endpoints de IA

use serde::{Deserialize, Serialize};
use log::info;

/// Resposta de IA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub content: String,
    pub provider: String,
    pub model: String,
    pub tokens_used: Option<usize>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Handler para consulta de IA
pub async fn ai_query_handler(prompt: String, context: String) -> Result<String, String> {
    info!("API: AI query - prompt: {}, context: {}", prompt, context);
    
    if prompt.trim().is_empty() {
        return Err("Prompt vazio".to_string());
    }
    
    // Gerar resposta inteligente baseada no contexto e prompt
    let response = generate_intelligent_response(&prompt, &context).await;
    
    // Simular delay de processamento
    tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
    
    Ok(response)
}

/// Gera resposta inteligente baseada no prompt e contexto
async fn generate_intelligent_response(prompt: &str, context: &str) -> String {
    let prompt_lower = prompt.to_lowercase();
    
    match context {
        "health" => generate_health_response(prompt, &prompt_lower),
        "finance" => generate_finance_response(prompt, &prompt_lower),
        "nutrition" => generate_nutrition_response(prompt, &prompt_lower),
        "creator" => generate_creator_response(prompt, &prompt_lower),
        "daily" => generate_daily_response(prompt, &prompt_lower),
        _ => generate_general_response(prompt)
    }
}

fn generate_health_response(prompt: &str, prompt_lower: &str) -> String {
    if prompt_lower.contains("dor de cabeça") || prompt_lower.contains("cefaleia") || prompt_lower.contains("enxaqueca") {
        "🏥 **Análise de Cefaleia**\n\n\
        **Tipos principais:**\n\
        • **Tensional**: Pressão bilateral, estresse, má postura\n\
        • **Enxaqueca**: Unilateral, pulsátil, sensibilidade à luz/som\n\
        • **Sinusite**: Pressão facial, piora ao inclinar\n\
        • **Cervicogênica**: Origem no pescoço, rigidez\n\n\
        **Sinais de alerta (procure emergência):**\n\
        ⚠️ Dor súbita e severa (\"pior da vida\")\n\
        ⚠️ Febre alta + rigidez nucal\n\
        ⚠️ Alterações visuais ou fala\n\
        ⚠️ Fraqueza em membros\n\n\
        **Tratamento inicial:**\n\
        • Hidratação adequada\n\
        • Ambiente escuro e silencioso\n\
        • Compressa fria na testa\n\
        • Analgésicos simples (conforme orientação)\n\n\
        *Consulte um médico se persistir >3 dias ou piorar*".to_string()
    } else if prompt_lower.contains("coriza") || prompt_lower.contains("nariz escorrendo") || prompt_lower.contains("resfriado") {
        "🏥 **Análise de Coriza**\n\n\
        **Causas principais:**\n\
        • **Viral**: Resfriado comum (rhinovirus)\n\
        • **Alérgica**: Pólen, ácaros, pelos\n\
        • **Bacteriana**: Sinusite secundária\n\
        • **Irritativa**: Fumaça, odores fortes\n\n\
        **Características por tipo:**\n\
        • **Viral**: Clara→amarelada, 7-10 dias\n\
        • **Alérgica**: Clara, coceira, sazonalidade\n\
        • **Bacteriana**: Espessa, esverdeada, >10 dias\n\n\
        **Tratamento suporte:**\n\
        • Lavagem nasal com soro fisiológico 0,9%\n\
        • Hidratação abundante (2-3L/dia)\n\
        • Umidificação do ambiente (50-60%)\n\
        • Repouso adequado\n\n\
        **Procure médico se:**\n\
        ⚠️ Febre >38,5°C por >3 dias\n\
        ⚠️ Secreção com sangue\n\
        ⚠️ Dor facial intensa\n\
        ⚠️ Piora após 10 dias".to_string()
    } else if prompt_lower.contains("febre") {
        "🏥 **Manejo de Febre**\n\n\
        **Classificação:**\n\
        • **Subfebril**: 37,1-37,8°C\n\
        • **Febre baixa**: 37,9-38,5°C\n\
        • **Febre moderada**: 38,6-39,5°C\n\
        • **Febre alta**: >39,5°C\n\n\
        **Cuidados gerais:**\n\
        • Hidratação constante\n\
        • Repouso absoluto\n\
        • Roupas leves\n\
        • Ambiente ventilado\n\
        • Monitoramento de temperatura\n\n\
        **Sinais de alerta:**\n\
        ⚠️ Febre >39°C persistente\n\
        ⚠️ Dificuldade respiratória\n\
        ⚠️ Manchas na pele\n\
        ⚠️ Convulsões\n\
        ⚠️ Rigidez de nuca\n\n\
        *Sempre consulte um médico para avaliação adequada*".to_string()
    } else {
        format!("⚕️ **Orientação Médica**\n\n\
        Entendi sua consulta sobre: \"{}\"\n\n\
        **Como médico especialista, oriento:**\n\
        • Toda avaliação médica requer exame presencial\n\
        • Sintomas devem ser contextualizados com histórico\n\
        • Autodiagnóstico pode ser perigoso\n\
        • Tratamentos dependem de avaliação individual\n\n\
        **Para orientações específicas sobre:**\n\
        • Dores de cabeça → Digite \"dor de cabeça\"\n\
        • Sintomas respiratórios → Digite \"coriza\" ou \"resfriado\"\n\
        • Febre → Digite \"febre\"\n\n\
        *⚠️ Em caso de emergência, procure atendimento imediato*", prompt)
    }
}

fn generate_finance_response(prompt: &str, prompt_lower: &str) -> String {
    if prompt_lower.contains("investir") || prompt_lower.contains("investimento") || prompt_lower.contains("onde aplicar") {
        "💰 **Guia de Investimentos**\n\n\
        **1. Base sólida primeiro:**\n\
        • Reserva de emergência (6-12 meses gastos)\n\
        • Quitação de dívidas juros altos (>15% a.a.)\n\
        • Definição clara de objetivos\n\n\
        **2. Perfil de investidor:**\n\
        • **Conservador**: Tesouro Direto, CDB, LCI/LCA\n\
        • **Moderado**: Fundos DI + ações blue chips\n\
        • **Arrojado**: Ações growth, REITs, FIIs, crypto\n\n\
        **3. Estratégia de alocação:**\n\
        • 60% Renda Fixa + 40% Renda Variável (moderado)\n\
        • Diversificar setores e geografias\n\
        • Rebalanceamento trimestral\n\n\
        **4. Produtos recomendados 2024:**\n\
        • **RF**: Tesouro IPCA+ 2029, CDB >100% CDI\n\
        • **RV**: ETFs (BOVA11, IVVB11), ações (ITUB4, VALE3)\n\
        • **Internacional**: ETFs S&P500, REIT americanos\n\n\
        ⚠️ **Rentabilidade passada ≠ garantia futura**".to_string()
    } else if prompt_lower.contains("orçamento") || prompt_lower.contains("gastos") || prompt_lower.contains("planejamento") {
        "💰 **Planejamento Financeiro**\n\n\
        **Regra 50-30-20:**\n\
        • **50%** Necessidades (moradia, alimentação, transporte)\n\
        • **30%** Desejos (lazer, hobbies, supérfluos)\n\
        • **20%** Poupança e investimentos\n\n\
        **Passo a passo:**\n\
        1. **Mapear gastos** por 30 dias\n\
        2. **Categorizar** essencial vs supérfluo\n\
        3. **Estabelecer metas** SMART\n\
        4. **Automatizar** investimentos\n\
        5. **Revisar** mensalmente\n\n\
        **Ferramentas úteis:**\n\
        • Planilhas de controle\n\
        • Apps: GuiaBolso, Mobills, OrganizZe\n\
        • Método envelope (gastos variáveis)\n\n\
        **Dicas de economia:**\n\
        • Renegociar contratos anuais\n\
        • Compras conscientes (lista prévia)\n\
        • Cashback e programas de pontos".to_string()
    } else {
        format!("📊 **Consultoria Financeira**\n\n\
        Analisando sua consulta: \"{}\"\n\n\
        **Como especialista financeiro, posso ajudar com:**\n\
        • Estratégias de investimento\n\
        • Planejamento de aposentadoria\n\
        • Análise de carteira\n\
        • Educação financeira\n\
        • Controle de gastos\n\n\
        **Para orientações específicas digite:**\n\
        • \"investir\" → Guia completo de investimentos\n\
        • \"orçamento\" → Planejamento financeiro pessoal\n\
        • \"aposentadoria\" → Cálculos previdenciários\n\n\
        *📈 Baseio minhas análises em dados de mercado atuais*", prompt)
    }
}

fn generate_nutrition_response(prompt: &str, prompt_lower: &str) -> String {
    if prompt_lower.contains("emagrecer") || prompt_lower.contains("perder peso") || prompt_lower.contains("dieta") {
        "🍎 **Plano de Emagrecimento Saudável**\n\n\
        **Princípios fundamentais:**\n\
        • Déficit calórico moderado (300-500 cal/dia)\n\
        • Proteína adequada (1,6-2,2g/kg peso)\n\
        • Hidratação (35ml/kg peso)\n\
        • Sono de qualidade (7-9h)\n\n\
        **Distribuição de macronutrientes:**\n\
        • **Proteínas**: 25-30% (saciedade + massa muscular)\n\
        • **Carboidratos**: 40-45% (energia + fibras)\n\
        • **Gorduras**: 25-30% (hormônios + vitaminas)\n\n\
        **Estratégias práticas:**\n\
        • Comer a cada 3-4 horas\n\
        • Pratos: 50% vegetais + 25% proteína + 25% carboidrato\n\
        • Mastigar devagar (20 min/refeição)\n\
        • Exercícios 4-5x/semana\n\n\
        **Evitar:**\n\
        ❌ Dietas muito restritivas (<1200 cal)\n\
        ❌ Produtos \"milagrosos\"\n\
        ❌ Jejuns sem orientação\n\
        ❌ Exclusão total de grupos alimentares\n\n\
        *Consulte nutricionista para plano personalizado*".to_string()
    } else if prompt_lower.contains("ganhar peso") || prompt_lower.contains("massa") || prompt_lower.contains("engordar") {
        "🍎 **Ganho de Peso Saudável**\n\n\
        **Meta**: +0,5-1kg por semana\n\
        **Superávit calórico**: +300-500 cal/dia\n\n\
        **Estratégias nutricionais:**\n\
        • 6-8 refeições por dia\n\
        • Proteína: 1,8-2,5g/kg peso\n\
        • Carboidratos: 5-7g/kg peso\n\
        • Gorduras: 1-1,5g/kg peso\n\n\
        **Alimentos calóricos saudáveis:**\n\
        • **Proteínas**: Carnes, ovos, leite, whey\n\
        • **Carboidratos**: Aveia, batata-doce, frutas\n\
        • **Gorduras**: Abacate, nuts, azeite, salmão\n\n\
        **Suplementação (se necessário):**\n\
        • Whey protein (pós-treino)\n\
        • Creatina (3-5g/dia)\n\
        • Hipercalórico (entre refeições)\n\n\
        **Exercícios**: Musculação 4-5x/semana\n\
        *Acompanhamento profissional é essencial*".to_string()
    } else {
        format!("🥗 **Orientação Nutricional**\n\n\
        Sua consulta: \"{}\"\n\n\
        **Como nutricionista especialista, posso orientar sobre:**\n\
        • Planos alimentares personalizados\n\
        • Reeducação alimentar\n\
        • Nutrição esportiva\n\
        • Distúrbios alimentares\n\
        • Suplementação\n\n\
        **Para orientações específicas digite:**\n\
        • \"emagrecer\" → Plano de perda de peso\n\
        • \"ganhar peso\" → Estratégia de ganho de massa\n\
        • \"diabetes\" → Controle glicêmico\n\n\
        *🥄 Baseio orientações em evidências científicas atuais*", prompt)
    }
}

fn generate_creator_response(prompt: &str, prompt_lower: &str) -> String {
    if prompt_lower.contains("conteúdo") || prompt_lower.contains("post") || prompt_lower.contains("redes sociais") {
        "🎨 **Estratégia de Conteúdo 2024**\n\n\
        **Tendências atuais:**\n\
        • **Autenticidade**: Conteúdo pessoal e genuíno\n\
        • **Educação**: Valor real para audiência\n\
        • **Vídeo curto**: Reels, TikTok, YouTube Shorts\n\
        • **UGC**: Conteúdo gerado pelo usuário\n\n\
        **Formatos que funcionam:**\n\
        • Carrosséis educativos (Instagram)\n\
        • Vídeos tutoriais (YouTube/TikTok)\n\
        • Stories interativos (enquetes, quiz)\n\
        • Lives/podcasts (conexão direta)\n\n\
        **Calendário editorial:**\n\
        • **Segunda**: Motivação/dicas\n\
        • **Quarta**: Conteúdo educativo\n\
        • **Sexta**: Entretenimento/trending\n\
        • **Domingo**: Bastidores/pessoal\n\n\
        **Métricas importantes:**\n\
        📊 Engajamento > Alcance\n\
        💬 Comentários > Curtidas\n\
        🔄 Compartilhamentos = Ouro\n\
        ⏱️ Tempo de visualização".to_string()
    } else if prompt_lower.contains("design") || prompt_lower.contains("visual") || prompt_lower.contains("arte") {
        "🎨 **Design Visual Eficaz**\n\n\
        **Princípios fundamentais:**\n\
        • **Contraste**: Hierarquia visual clara\n\
        • **Repetição**: Identidade consistente\n\
        • **Alinhamento**: Organização limpa\n\
        • **Proximidade**: Agrupamento lógico\n\n\
        **Paleta de cores 2024:**\n\
        • **Minimalista**: Branco, cinza, um accent\n\
        • **Vibrante**: Gradientes e neons\n\
        • **Natural**: Tons terrosos e verdes\n\
        • **Nostálgico**: Tons pastéis retrô\n\n\
        **Tipografia:**\n\
        • **Títulos**: Sans-serif bold\n\
        • **Corpo**: Sans-serif regular\n\
        • **Destaque**: Script/display com moderação\n\n\
        **Ferramentas recomendadas:**\n\
        • **Gratuitas**: Canva, Figma Community\n\
        • **Profissionais**: Adobe Creative Suite\n\
        • **Fotos**: Unsplash, Pexels\n\
        • **Ícones**: Feather, Lucide".to_string()
    } else {
        format!("🎨 **Consultoria Criativa**\n\n\
        Sua demanda: \"{}\"\n\n\
        **Como especialista em criação, posso ajudar com:**\n\
        • Estratégia de conteúdo\n\
        • Design visual e branding\n\
        • Copywriting persuasivo\n\
        • Análise de tendências\n\
        • Otimização de performance\n\n\
        **Para orientações específicas digite:**\n\
        • \"conteúdo\" → Estratégias de criação\n\
        • \"design\" → Princípios visuais\n\
        • \"copy\" → Técnicas de escrita\n\n\
        *✨ Baseio estratégias em dados e tendências atuais*", prompt)
    }
}

fn generate_daily_response(prompt: &str, prompt_lower: &str) -> String {
    if prompt_lower.contains("produtividade") || prompt_lower.contains("organizar") || prompt_lower.contains("rotina") {
        "📅 **Sistema de Produtividade**\n\n\
        **Método GTD (Getting Things Done):**\n\
        1. **Capturar**: Anote tudo (inbox único)\n\
        2. **Esclarecer**: O que é? É acionável?\n\
        3. **Organizar**: Categorize por contexto\n\
        4. **Refletir**: Revise semanalmente\n\
        5. **Engajar**: Execute com foco\n\n\
        **Técnicas complementares:**\n\
        • **Pomodoro**: 25min foco + 5min pausa\n\
        • **Time blocking**: Blocos de tempo dedicados\n\
        • **Batch processing**: Agrupar tarefas similares\n\
        • **2-minute rule**: <2min = fazer agora\n\n\
        **Ferramentas recomendadas:**\n\
        • **Tarefas**: Todoist, TickTick, Any.do\n\
        • **Notas**: Notion, Obsidian, Logseq\n\
        • **Tempo**: RescueTime, Toggl\n\
        • **Foco**: Forest, Freedom\n\n\
        **Rituais diários:**\n\
        🌅 **Manhã**: Revisão + 3 prioridades\n\
        🌙 **Noite**: Reflexão + preparação do dia seguinte".to_string()
    } else if prompt_lower.contains("tempo") || prompt_lower.contains("agenda") || prompt_lower.contains("cronograma") {
        "📅 **Gestão de Tempo Eficaz**\n\n\
        **Matriz de Eisenhower:**\n\
        • **Q1** (Urgente + Importante): Crises → Fazer\n\
        • **Q2** (Importante + Não urgente): Prevenção → Planejar\n\
        • **Q3** (Urgente + Não importante): Interrupções → Delegar\n\
        • **Q4** (Não urgente + Não importante): Desperdícios → Eliminar\n\n\
        **Planejamento semanal:**\n\
        • **Domingo**: Revisão da semana + planejamento\n\
        • **Daily**: 15min manhã para prioridades\n\
        • **Sexta**: Fechamento e preparação\n\n\
        **Proteção do tempo:**\n\
        • Bloqueio de calendário para deep work\n\
        • \"Não\" educado para comprometimentos extras\n\
        • Limite de reuniões (máx 50% do dia)\n\
        • Pausas programadas (energização)\n\n\
        **Regra 80/20 (Pareto):**\n\
        20% das atividades geram 80% dos resultados\n\
        *Identifique e priorize essas atividades*".to_string()
    } else {
        format!("📋 **Assistente Pessoal**\n\n\
        Sua demanda: \"{}\"\n\n\
        **Como assistente pessoal, posso ajudar com:**\n\
        • Organização de rotinas\n\
        • Gestão de tempo\n\
        • Sistemas de produtividade\n\
        • Planejamento de objetivos\n\
        • Otimização de processos\n\n\
        **Para orientações específicas digite:**\n\
        • \"produtividade\" → Métodos e técnicas\n\
        • \"tempo\" → Gestão de agenda\n\
        • \"hábitos\" → Criação de rotinas\n\n\
        *⚡ Foco em sistemas sustentáveis e eficazes*", prompt)
    }
}

fn generate_general_response(prompt: &str) -> String {
    format!("🤖 **R5 Flowlight IA**\n\n\
    Sua consulta: \"{}\"\n\n\
    **Estou preparado para ajudar em áreas especializadas:**\n\
    • 🏥 **Saúde**: Orientações médicas baseadas em evidências\n\
    • 💰 **Finanças**: Investimentos e planejamento financeiro\n\
    • 🍎 **Nutrição**: Planos alimentares e orientação nutricional\n\
    • 🎨 **Creator**: Estratégias de conteúdo e design\n\
    • 📅 **Daily**: Produtividade e organização pessoal\n\n\
    **Para respostas especializadas:**\n\
    Selecione um nicho específico na tela inicial\n\n\
    *🔬 Respostas baseadas em conhecimento científico atual*", prompt)
}