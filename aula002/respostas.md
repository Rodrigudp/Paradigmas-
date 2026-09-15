# Atividade de Sala - Resolução da Lista de Exercícios (Capítulo 2)

Este documento apresenta a resolução de 10 questões da lista de exercícios contida no arquivo `exercises.pdf`, sendo 5 questões selecionadas entre as 10 primeiras e 5 questões selecionadas entre as 10 últimas. As respostas são estritamente fundamentadas no Capítulo 2 do livro **"Conceitos de Linguagens de Programação" (11ª Edição)** de Robert Sebesta.

---

## Parte I: Questões Selecionadas entre as 10 Primeiras (Q1 a Q10)

### 1. A genealogia das linguagens não é uma escada de progresso. Explique essa afirmação e apresente dois fatores históricos que fazem uma linguagem influenciar outra sem necessariamente substituí-la.
* **Justificativa:** A história das linguagens de programação, ilustrada pela complexa genealogia do livro [123, 124], diz que a evolução tecnológica não ocorre de forma linear ou em uma "escada" onde cada degrau substitui o anterior. Linguagens consolidadas continuam coexistindo ativamente com seus descendentes devido a especificidades de nicho, investimentos legados e contextos ecológicos.
* **Fatores históricos de influência sem substituição:**
  1. **Arquitetura de Hardware e Paradigma Imperativo:** A forte influência da arquitetura clássica de von Neumann ditou que a maioria das linguagens de sucesso comercial fossem imperativas (baseadas em variáveis, atribuição e loops) [80]. Por exemplo, embora o **Lisp** (1958) tenha sido pioneiro no paradigma funcional com recursos extremamente elegantes, ele não substituiu o **Fortran** no desenvolvimento científico devido à ineficiência prática das primeiras implementações interpretadas em relação à execução direta no hardware da época [130, 131, 141].
  2. **Patrocínio Institucional e Domínios de Aplicação:** Muitas linguagens nascem ou ganham tração massiva por exigência de grandes organizações ou para atender a domínios muito específicos. O **COBOL** (1960) dominou o processamento comercial por décadas devido ao apoio governamental e corporativo, resistindo a linguagens posteriores conceitualmente mais modernas [153, 155]. Da mesma forma, o esforço bilionário do Departamento de Defesa americano para criar o **Ada** exemplifica como necessidades de sistemas críticos integrados mantêm linguagens especializadas sem que estas substituam outras no uso comercial geral [181].

---

### 2. Plankalkül não foi implementada em sua época. Ainda assim, por que ela é relevante para a história das linguagens? Cite três recursos antecipados por seu projeto e explique o valor de um deles.
* **Relevância histórica:** Projetada pelo pioneiro alemão Konrad Zuse entre 1937 e 1945 em condições de isolamento quase total durante a guerra, a **Plankalkül** [125] provou de forma teórica e prática que algoritmos de alta complexidade (incluindo 49 páginas para jogar xadrez) [126] podiam ser expressos em notações abstratas e independentes de máquina de alto nível muito antes de computadores eletrônicos comerciais ou compiladores existirem.
* **Três recursos antecipados:**
  1. **Estruturas de Dados Avançadas:** Suporte nativo para matrizes (arrays) multidimensionais e tipos compostos semelhantes a registros (structures) [126, 226].
  2. **Estruturas de Controle Iterativas e Condicionais:** Loops aninhados e condicionais complexos.
  3. **Assertivas Matemáticas de Estado:** Inclusão de asserções que expressam o que deve ser verdadeiro sobre as variáveis do programa em pontos específicos da execução [125].
* **O valor das Assertivas Matemáticas:** Este recurso antecipou em décadas o conceito moderno de programação defensiva, asserções de tempo de execução (como as de Java [125]) e, de forma mais profunda, as bases da **semântica axiomática** [125]. Ele permitiu que o próprio código de alto nível contivesse especificações matemáticas formais de corretude, um dos pilares mais avançados de engenharia de software crítica hoje.

---

### 4. Explique por que o projeto Fortran precisou convencer programadores de que código traduzido podia competir com código de máquina escrito à mão. Relacione desempenho, custo de programação e adoção.
* **Necessidade de convencimento:** Nos primórdios da computação, na década de 1950, o tempo de processamento dos computadores era extremamente caro e limitado em memória, enquanto os primeiros esforços de codificação automática (como o *Short Code*) eram interpretados de forma pura e funcionavam cerca de **50 vezes mais devagar** que o código de máquina nativo [130, 131]. Os programadores viam sistemas de alto nível como brinquedos ineficientes.
* **Relação entre Desempenho, Custo e Adoção:**
  1. **Desempenho:** Para viabilizar a adoção do **Fortran**, a equipe liderada por John Backus sabia que o compilador precisaria gerar códigos de máquina cuja eficiência em tempo de execução fosse quase idêntica à de códigos escritos à mão por especialistas [131]. O projeto do primeiro compilador Fortran focou quase obsessivamente na otimização de código por esse motivo.
  2. **Custo de Programação:** Embora o tempo do programador estivesse se tornando uma preocupação, a economia financeira do desenvolvimento (escrever programas em poucas horas em vez de semanas [131]) só seria atraente se a execução do programa compilado não consumisse recursos excessivos de máquina em hardware que operava na faixa dos milissegundos por instrução [131].
  3. **Adoção:** O sucesso estrondoso e a dominação histórica do Fortran I [135, 224] provaram que Backus estava correto: ao entregar um código gerado de altíssima eficiência, a barreira do preconceito dos programadores foi rompida, permitindo que a drástica redução nos custos de desenvolvimento impulsionasse a adoção em massa [131, 135].

---

### 5. Lisp surgiu em um contexto diferente de Fortran. Compare os domínios, a representação de dados e o estilo de computação favorecido pelas duas linguagens.
* **Comparação detalhada:**

| Critério | **Fortran** | **Lisp** |
| :--- | :--- | :--- |
| **Domínio Primário** | Computação científica e matemática complexa (engenharia, cálculos físicos) utilizando hardware voltado para cálculos numéricos (IBM 704) [135, 136, 224]. | Inteligência Artificial (IA) nascente e computação simbólica (processamento de linguagens naturais, diferenciações algébricas) [136, 137, 225]. |
| **Representação de Dados** | Matrizes (arrays) de tamanho fixo, variáveis numéricas escalares (inteiros e pontos flutuantes) de forma fortemente orientada à memória de máquina [131, 135]. | Átomos (símbolos e literais) e listas encadeadas (dinâmicas e flexíveis) [138]. Dados e códigos compartilham exatamente a mesma estrutura (sintaxe de expressões-S) [140]. |
| **Estilo de Computação** | **Paradigma Imperativo** (estruturas de controle sequenciais, loops como o comando `Do` e alterações diretas no estado da memória via atribuição) [135, 139]. | **Paradigma Funcional** (aplicação estrita de funções sobre argumentos, uso extensivo de recursão e controle condicional sem necessidade de variáveis mutáveis ou loops) [139]. |

---

### 6. Avalie três contribuições de ALGOL 60 que ultrapassaram sua adoção comercial. Por que uma linguagem pode ser muito influente sem dominar o mercado?
* **Três contribuições duradouras do ALGOL 60:**
  1. **Descrição Sintática Formal (BNF):** Foi a primeira linguagem que teve sua sintaxe totalmente descrita usando a Forma de Backus-Naur (BNF), o que deu origem a ramos fundamentais da ciência da computação (teoria de compiladores e linguagens formais) [149, 151].
  2. **Estrutura de Blocos e Escopo Local:** Introduziu o conceito de modularização local utilizando blocos (delimitações que definem novos ambientes de dados temporários e locais para variáveis dinâmicas de pilha) [150].
  3. **Recursão de Procedimentos e Matrizes Dinâmicas de Pilha:** Estabeleceu a recursão como um padrão estruturado nas linguagens imperativas e permitiu matrizes cujo tamanho é determinado na alocação da pilha durante a execução [150].
* **Influência conceitual vs. Dominação de mercado:** O **ALGOL 60** dominou o meio acadêmico e as discussões teóricas, influenciando quase todas as linguagens imperativas subsequentes (como Pascal, Ada e C) [151, 173]. No entanto, fracassou comercialmente devido a decisões pragmáticas ruins: a ausência de comandos padrão para Entrada e Saída (I/O) nativos na linguagem — delegados para serem implementados de forma distinta em cada máquina, o que prejudicou enormemente a portabilidade [226] — e a recusa da IBM em apoiá-la plenamente para proteger seu investimento massivo no Fortran [59].

---

## Parte II: Questões Selecionadas entre as 10 Últimas (Q11 a Q20)

### 11. Construa uma cadeia de influência que passe por ALGOL, Pascal e C. Depois contraste essa linhagem imperativa com a proposta declarativa de Prolog.
* **Cadeia de Influência Imperativa:**
  1. **ALGOL 60** foi a grande fundadora do estilo imperativo moderno com suas sentenças de controle estruturadas, recursão e estrutura de blocos [150].
  2. **Pascal** (1971), projetado por Niklaus Wirth (que participou ativamente de comitês de evolução do ALGOL [173]), nasceu de uma proposta modificada deste (ALGOL-W) [174]. Ele herdou a elegância e a estrutura de blocos, mas focou intensamente na simplicidade de projeto e na forte verificação de tipos para servir como ferramenta de ensino segura [173, 174].
  3. **C** (1972), concebido por Dennis Ritchie para o UNIX, também descende dos conceitos de controle e estruturação do ALGOL (e ALGOL 68) [176], mas removeu a rigidez de segurança de tipos do Pascal em prol de flexibilidade extrema de baixo nível, herdando características de linguagens de sistemas não tipadas (BCPL e B) [176, 177].
* **Contraste com a Proposta Declarativa do Prolog:**
  * **Linhagem Imperativa (ALGOL, Pascal, C):** O programador escreve algoritmos em termos procedimentais: dita *como* o computador deve processar os dados por meio de instruções passo a passo e modificação contínua de posições de memória (variáveis) [88].
  * **Prolog (Linguagem Declarativa/Lógica):** O programador não escreve o procedimento de execução. Em vez disso, ele define um conjunto de fatos e regras de lógica formal (cálculo de predicados) sobre as entidades [179]. O sistema Prolog usa uma técnica automática de inferência matemática de prova de teoremas (Resolução e Unificação) para responder a consultas, processando o fluxo de forma não procedural [179].

---

### 13. Ada resultou de requisitos e projeto em grande escala. Analise como confiabilidade, tipos, pacotes e concorrência se relacionam ao domínio de sistemas críticos.
* **Contexto de Domínio:** O **Ada** foi contratado pelo Departamento de Defesa americano (DoD) para ser a linguagem padrão de sistemas embarcados [181]. Esses sistemas gerenciam mísseis, radares, aeronaves e equipamentos de defesa, onde qualquer falha de software custa vidas humanas e bilhões de dólares.
* **Relação com Recursos de Linguagem:**
  1. **Confiabilidade:** O processo de projeto foi extremamente competitivo e rígido [181-183]. A sintaxe foi feita para ser extremamente explícita e reduzir ambiguous, exigindo, por exemplo, fechamentos distintos para cada estrutura de controle (como `end if` e `end loop`) para evitar erros comuns de aninhamento [74].
  2. **Verificação de Tipos Rígida:** Introduziu tipagem estática extremamente forte para pegar o máximo de erros em tempo de compilação [181]. Ele suporta até mesmo tipos de subfaixa que limitam numericamente os valores válidos para uma variável.
  3. **Pacotes:** O suporte linguístico a **pacotes** permitiu o encapsulamento robusto de estruturas de dados e suas operações (abstração de dados de alto nível), permitindo compilações separadas e desenvolvimento cooperativo seguro de softwares militares colossais [184].
  4. **Concorrência Integrada:** Para lidar com sistemas de tempo real com eventos físicos simultâneos, o Ada embutiu concorrência na própria linguagem via **tarefas** (`tasks`) sincronizadas pelo mecanismo de *rendezvous* [186], garantindo controle estruturado que não dependia exclusivamente de APIs de sistemas operacionais específicos.

---

### 14. Compare o papel dos objetos em Smalltalk, C++ e Java. Inclua na resposta o compromisso de C++ com C e a estratégia de portabilidade de Java.
* **Smalltalk:** Adota o modelo puro de orientação a objetos [192]. Nele, **tudo é um objeto** (desde um inteiro escalar até estruturas complexas) [192, 498]. Toda computação é realizada de maneira estritamente uniforme por meio do envio de mensagens [192]. Não existem tipos primitivos fora do sistema de classes.
* **C++:** Projetado por Bjarne Stroustrup como uma extensão compatível do C ("C com Classes") [196, 197]. Ele assumiu um compromisso rígido de compatibilidade reversa para reuso de código imperativo legado [196, 199]. É uma linguagem híbrida: mantém tipos primitivos nativos eficientes e programação procedural, ao mesmo tempo que adiciona classes e herança [196, 199, 501].
* **Java:** Nasceu para simplificar e tornar o desenvolvimento C++ mais seguro e portátil [201]. Java removeu recursos perigosos do C++ (como ponteiros físicos diretos, herança múltipla de classes e sobrecarga de operadores) [201, 209]. É híbrida (mantém tipos primitivos para eficiência), mas todas as classes são alocadas no monte e limpas por coleta de lixo implícita [517].
  * **Estratégia de Portabilidade:** Java é compilada para uma linguagem intermediária universal de baixo nível chamada **bytecode**, a qual é executada em qualquer plataforma que tenha uma Máquina Virtual Java (JVM) [208, 209].

---

### 15. A primeira aplicação de Java não foi a Web, mas a Web impulsionou sua adoção. Explique como mudanças de contexto podem reposicionar uma linguagem.
* **A primeira aplicação de Java:** Desenvolvida originalmente no início dos anos 1990 pela Sun Microsystems sob o codinome "Green Project", a linguagem foi projetada para dispositivos eletrônicos de consumo e televisores interativos a cabo [229, 513]. O mercado não estava maduro e essa primeira aplicação foi um fracasso comercial.
* **A mudança de contexto e o reposicionamento:**
  1. No meio da década de 1990, a World Wide Web explodiu comercialmente. Os documentos HTML da época eram totalmente estáticos, e havia uma necessidade desesperada de rodar conteúdos interativos e dinâmicos de forma segura nos computadores dos usuários [203].
  2. As características originais do Java — ser extremamente pequena, consumir poucos recursos, possuir segurança contra acessos à memória (sem ponteiros) e ter arquitetura neutra de compilação em bytecode (JVM) [201, 208] — tornaram-na a candidata ideal para a Web.
  3. A Sun reposicionou a linguagem e introduziu os **Java Applets** dentro dos navegadores [203]. A Web forneceu o ecossistema perfeito de adoção e resgatou a linguagem do esquecimento, provando que o sucesso comercial de uma tecnologia de linguagem depende do alinhamento oportuno com as demandas de mercado do momento.

---

### 16. Compare Perl, JavaScript, PHP, Python, Ruby e Lua usando três eixos: domínio inicial, estruturas de dados e estratégia de implementação. Evite concluir que todas são iguais por serem chamadas de scripting.
* **Comparação Sistemática das Linguagens de Scripting:**

| Linguagem | **Eixo 1: Domínio Inicial** | **Eixo 2: Estruturas de Dados** | **Eixo 3: Estratégia de Implementação** |
| :--- | :--- | :--- | :--- |
| **Perl** | Processamento de relatórios de texto de grande escala e automação de tarefas de sistemas UNIX (substituindo scripts sh, awk e sed) [230]. | Variáveis escalares simples, matrizes e matrizes associativas nativas altamente eficientes chamadas **hashes** [230, 469]. | Tradicionalmente compilada de forma rápida para uma representação intermediária e interpretada por software (híbrido) [202, 231, 236]. |
| **JavaScript** | Scripting do lado cliente em navegadores web para controle dinâmico e modificações em tempo de execução de documentos HTML [203, 204, 230]. | Suporte a objetos dinâmicos (propriedades adicionadas em tempo de execução) [204, 230]. | Interpretada de forma pura no navegador do cliente (atualmente com compiladores JIT integrados) [204, 331, 332]. |
| **PHP** | Scripting dinâmico executado do lado do servidor web, tipicamente integrado ao HTML para acesso a bancos de dados [202, 230]. | Uso proeminente de matrizes associativas dinâmicas que combinam comportamento de arrays e hashes [230]. | Interpretado de forma direta pelo interpretador embutido no servidor web [202]. |
| **Python** | Scripting geral focado em legibilidade máxima, desenvolvimento rápido e suporte multiparadigma [230]. | Substitui os vetores clássicos por listas dinâmicas, tuplas (imutáveis) e dicionários (hashes) nativos [230, 475, 479]. | Compilada para um formato intermediário de bytecode (.pyc) e interpretada por uma máquina virtual de software [230, 231]. |
| **Ruby** | Scripting de propósito geral projetado como uma alternativa mais limpa e totalmente OO ao Perl e ao Python [205]. | Orientação a objetos pura (tudo, de strings a números, é um objeto real) com suporte nativo a arrays e hashes [205, 231]. | Interpretada de forma dinâmica [205]. |
| **Lua** | Linguagem leve de extensão embarcada para fornecer caminhos flexíveis de customização a outros sistemas (como engines de jogos) [205]. | Baseada de forma quase exclusiva em uma única e flexível estrutura de dados: a **tabela** (table) [206]. | Traduzida de forma rápida para um formato intermediário compacto de bytecode e interpretada em uma máquina virtual ultraleve (cerca de 150 KB) [207]. |

---
