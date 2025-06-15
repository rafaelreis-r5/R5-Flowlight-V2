//! Prompt templates for AI modules
//!
//! Templates de prompts para módulos de IA

use crate::modules::ModuleType;

/// Templates de prompts por módulo
pub struct PromptTemplates;

impl PromptTemplates {
    /// Obtém o prompt do sistema para um módulo
    pub fn get_system_prompt(module_type: &ModuleType) -> String {
        match module_type {
            ModuleType::Finance => {
                "Você é um especialista em análise financeira e investimentos. \
                Forneça informações precisas, atualizadas e baseadas em dados de mercado. \
                Sempre inclua disclaimers sobre riscos de investimento.".to_string()
            },
            ModuleType::Health => {
                "Você é um assistente médico especializado em medicina clínica e do trabalho. \
                Forneça informações baseadas em evidências científicas e protocolos médicos atuais. \
                Sempre enfatize a importância de consulta médica presencial.".to_string()
            },
            ModuleType::Nutrition => {
                "Você é um especialista em nutrição e dietética. \
                Forneça informações baseadas em evidências científicas atuais. \
                Considere necessidades individuais e recomende consulta com nutricionista.".to_string()
            },
            ModuleType::Creator => {
                "Você é um especialista em criação de conteúdo e marketing digital. \
                Forneça ideias criativas, estratégias de engagement e tendências atuais. \
                Mantenha-se atualizado com as melhores práticas de cada plataforma.".to_string()
            },
            ModuleType::Daily => {
                "Você é um assistente pessoal eficiente e útil. \
                Ajude com tarefas do dia a dia, cálculos, conversões e organização. \
                Seja conciso e prático nas suas respostas.".to_string()
            }
        }
    }

    /// Obtém um template de prompt para uma funcionalidade específica
    pub fn get_function_prompt(module_type: &ModuleType, function: &str) -> Option<String> {
        match (module_type, function) {
            (ModuleType::Finance, "portfolio") => Some("Forneça um resumo do portfólio, incluindo alocação de ativos, rentabilidade e sugestões de rebalanceamento.".to_string()),
            (ModuleType::Finance, "analysis") => Some("Analise os dados de mercado fornecidos e forneça insights sobre: tendências, riscos, oportunidades e recomendações estratégicas.".to_string()),
            (ModuleType::Health, "symptoms") => Some("Com base nos sintomas descritos, forneça informações sobre: possíveis causas, gravidade, quando procurar ajuda médica e primeiros cuidados.".to_string()),
            (ModuleType::Health, "diagnosis") => Some("Forneça um diagnóstico preliminar com base nas informações e oriente sobre próximos passos e cuidados.".to_string()),
            (ModuleType::Nutrition, "meal-plan") => Some("Crie um plano alimentar considerando: objetivos nutricionais, restrições alimentares, preferências e valor nutricional equilibrado.".to_string()),
            (ModuleType::Nutrition, "calories") => Some("Forneça informações sobre calorias e composição nutricional do alimento solicitado.".to_string()),
            (ModuleType::Creator, "content") => Some("Gere ideias de conteúdo criativas e relevantes considerando: público-alvo, plataforma, tendências atuais e objetivos de engagement.".to_string()),
            (ModuleType::Creator, "design") => Some("Sugira elementos de design e layout atraentes para o conteúdo, incluindo paleta de cores e tipografia.".to_string()),
            (ModuleType::Daily, "tasks") => Some("Crie uma lista de tarefas diárias prioritizadas com base nas atividades fornecidas.".to_string()),
            (ModuleType::Daily, "schedule") => Some("Organize um cronograma diário considerando compromissos, durações e intervalos apropriados.".to_string()),
            _ => None
        }
    }
}
