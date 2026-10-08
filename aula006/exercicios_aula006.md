# Exercícios de Fixação — Tipos de Dados (Aula 06)

* **Aluno:** Rodrigo Del Padre
* **RA:** 24022092-2
* **Curso:** Engenharia de Software (6º Semestre) — Turma B
* **Disciplina:** Paradigmas / Conceitos de Linguagens de Programação
* **Referência Teórica:** Robert Sebesta, *Conceitos de Linguagens de Programação* (11ª Edição), Capítulo 6.
* **Base dos Exercícios:** Questões de fixação "Preveja a saída" do Slide 21 (`Aula006_TiposDeDados.pptx`), códigos em `exemplos/08_exercicio/`, referências de execução (`exemploaula006.pdf`) e estudo aprofundado dos tópicos dos Slides 21 a 23.

---

## Questão 1 — JavaScript (Ponto Flutuante e Limites de Inteiros)

### Código Original (`q1.js`)
```javascript
// Exercício Aula 06 - Questão 1
// ANTES de executar: qual é a saída? O problema é detectado na compilação, na execução ou nunca?
console.log(0.1 * 3 === 0.3);
console.log(9007199254740993);
```

### (a) Saída do Programa
```text
false
9007199254740992
```

### (b) Quando o problema é detectado?
> **NUNCA**. Nem o compilador/transpilador nem a máquina virtual (V8) acusam qualquer advertência ou erro em tempo de compilação ou execução. O programa roda até o fim e produz valores corrompidos/inesperados silenciosamente.

### (c) Análise Conceitual e Fundamentação (Sebesta, Cap. 6)
1. **Representação IEEE 754 de Ponto Flutuante:** Em JavaScript, todos os números pertencentes ao tipo primitivo `number` são representados internamente como valores de ponto flutuante de precisão dupla em binário (padrão IEEE 754 de 64 bits), conforme abordado por Sebesta na seção 6.2.1.2.
   * Frações decimais simples como $0.1$ não possuem representação binária finita; tornam-se dízimas periódicas no sistema de base 2 ($0.0001100110011..._2$).
   * A operação `0.1 * 3` resulta em `0.30000000000000004`. Ao comparar com `0.3` via operador estrito `===`, o resultado é invariavelmente `false`.
2. **Estouro de Resolução do Safe Integer:** Na representação de 64 bits (1 bit de sinal, 11 de expoente e 52 de mantissa/fração normalizada, gerando 53 bits de significando efetivo), o maior número inteiro consecutivo e seguro representável sem perda de precisão é $2^{53} - 1 = 9.007.199.254.740.991$ (`Number.MAX_SAFE_INTEGER`).
   * Ao informar o literal $9.007.199.254.740.993$, o bit menos significativo não possui posição física na mantissa de 53 bits.
   * O hardware/interpretador aplica o algoritmo de arredondamento IEEE 754 para o par mais próximo (*round to nearest, ties to even*), convertendo o número silenciosamente para `9007199254740992`.
3. **Impacto em Engenharia de Software:** Cálculos contábeis ou transações bancárias não podem utilizar tipos de ponto flutuante binário primitivos sob risco de fraude ou desbalanceamento de centavos. Devem ser empregadas classes decimais exatas com escala arbitrária (como `BigDecimal` em Java ou representações inteiras em centavos/subcentavos).

---

## Questão 2 — Python (Cadeias de Caracteres: Unicode vs. Bytes)

### Código Original (`q2.py`)
```python
# Exercício Aula 06 - Questão 2
# ANTES de executar: qual é a saída?
palavra = "maçã"
print(len(palavra), len(palavra.encode()))  # encode() usa UTF-8 por padrão
```

### (a) Saída do Programa
```text
4 6
```

### (b) Quando o problema é detectado?
> **SEM PROBLEMA / NENHUM ERRO DETECTADO**. Não se trata de uma falha ou exceção, mas sim do comportamento formal e semanticamente correto da abstração do sistema de tipos em Python 3.

### (c) Análise Conceitual e Fundamentação (Sebesta, Cap. 6)
1. **Separação entre Texto Abstrato e Bytes Físicos:** Sebesta analisa em 6.2.3 e 6.3 a evolução das cadeias de caracteres desde o padrão ASCII de 7/8 bits até os padrões modernos universais (ISO 10646 e Unicode). O Python 3 implementa uma divisão categórica entre dois tipos:
   * `str`: Representa uma sequência lógica e abstrata de pontos de código Unicode (*code points*).
   * `bytes`: Representa uma sequência contígua e bruta de octetos (bytes de 8 bits).
2. **Execução:**
   * A chamada `len(palavra)` afere a quantidade de caracteres textuais: `"m"`, `"a"`, `"ç"`, `"ã"`, totalizando exatamente **4 caracteres**.
   * A chamada `palavra.encode()` converte a sequência abstrata para uma codificação binária em disco/rede, adotando **UTF-8** como padrão.
   * Na especificação UTF-8, caracteres da tabela ASCII padrão (como `'m'` e `'a'`) ocupam exatamente 1 byte cada ($0 \times 6D$ e $0 \times 61$).
   * Já os caracteres latinos estendidos `'ç'` e `'ã'` exigem uma sequência multibyte de 2 octetos cada:
     * `'ç'` $\rightarrow$ `\xc3\xa7` (2 bytes)
     * `'ã'` $\rightarrow$ `\xc3\xa3` (2 bytes)
   * A soma total de armazenamento é $1 + 1 + 2 + 2 = \mathbf{6\text{ bytes}}$.
3. **Lição Arquitetural:** Em sistemas de software distribuídos, dimensionar buffers de rede ou colunas de banco de dados (`VARCHAR`) baseando-se na contagem de caracteres sem prever a multiplicação de bytes do UTF-8 acarreta falhas graves de truncamento de dados (*data truncation*).

---

## Questão 3 — Go (Inteiros Sem Sinal e Aritmética Modular)

### Código Original (`q3.go`)
```go
// Exercício Aula 06 - Questão 3
// ANTES de executar: qual é a saída? O problema é detectado na compilação, na execução ou nunca?
package main

import "fmt"

func main() {
    var b byte = 255
    b++
    fmt.Println(b)
}
```

### (a) Saída do Programa
```text
0
```

### (b) Quando o problema é detectado?
> **NUNCA**. Ocorre um estouro de inteiro aritmético (*integer overflow*) silencioso, sem qualquer interrupção, erro ou sinalização pelo runtime do Go.

### (c) Análise Conceitual e Fundamentação (Sebesta, Cap. 6)
1. **Tipos Inteiros e Estouro (Sebesta 6.2.1.1):** Em Go, a palavra-chave `byte` é um alias formal para `uint8` (inteiro não-assinado de 8 bits), cuja faixa de representação numérica válida abrange estritamente o intervalo fechado $[0, 255]$ ($2^8 - 1$).
2. **Aritmética Modular de Baixo Custo:**
   * Embora o compilador do Go realize verificações rigorosas em literais e constantes numéricas em tempo de compilação (rejeitando atribuições diretas que excedam a faixa), para operações com variáveis em tempo de execução o Go optou por seguir o modelo de aritmética modular de complemento ($255 + 1 \equiv 0 \pmod{256}$).
   * Essa decisão de projeto privilegia o desempenho bruto em nível de instrução de máquina (dispensando instruções extras de verificação de flag de *carry/overflow* no processador após cada incremento).
3. **Consequência em Segurança da Informação:** O estouro de inteiro silencioso é um vetor clássico de vulnerabilidades (CWE-190). Em loops de alocação de buffer ou validações de índices, um incremento que "dá a volta" para zero pode burlar regras de checagem de tamanho, gerando comportamentos inesperados.

---

## Questão 4 — Java (Matrizes, Inicialização Padrão e Limites)

### Código Original (`Q4.java`)
```java
// Exercício Aula 06 - Questão 4
// ANTES de executar: qual é a saída? O problema é detectado na compilação, na execução ou nunca?
public class Main {
    public static void main(String[] args) {
        int[] v = new int[3];
        System.out.println(v[0]);
        System.out.println(v[3]);
    }
}
```

### (a) Saída do Programa
```text
0
Exception in thread "main" java.lang.ArrayIndexOutOfBoundsException: Index 3 out of bounds for length 3
    at Main.main(Main.java:6)
```

### (b) Quando o problema é detectado?
> **NA EXECUÇÃO (Runtime)**. O programa compila com sucesso, executa a primeira linha imprimindo `0`, e é interrompido no segundo acesso lançando uma exceção de limite.

### (c) Análise Conceitual e Fundamentação (Sebesta, Cap. 6)
1. **Tipos Matriz e Alocação Dinâmica no Heap (Sebesta 6.5):** Em Java, vetores são instâncias de objetos no heap. A instrução `new int[3]` aloca um array contíguo de 3 inteiros de 32 bits e inicializa automaticamente todas as posições com o valor neutro padrão do tipo (`0`). Logo, a leitura de `v[0]` produz legitimamente `0`.
2. **Checagem Dinâmica de Limites (*Subscript Bounds Checking*):**
   * O tamanho do vetor é fixado em 3 posições, indexadas em base 0: índices válidos $\{0, 1, 2\}$.
   * O índice `3` aponta para o quarto elemento imaginário, situando-se fora do intervalo alocado.
   * Enquanto linguagens como C e C++ não realizam verificações de limites para priorizar velocidade (permitindo ler ou corromper dados adjacentes na memória), a JVM injeta checagens de limite (*bound checks*) em tempo de execução.
   * Ao detectar o índice ilegítimo, a execução é abortada imediatamente com o lançamento de `java.lang.ArrayIndexOutOfBoundsException`.
3. **Confiabilidade:** Essa verificação em tempo de execução impede invasões e acessos indevidos a blocos vizinhos de memória, demonstrando o trade-off de Java onde a segurança e a confiabilidade prevalecem sobre frações mínimas de nanossegundos de desempenho.

---

## Questão 5 — Rust (Semântica de Posse, Movimento e Borrow Checker)

### Código Original (`q5.rs`)
```rust
// Exercício Aula 06 - Questão 5
// ANTES de executar: qual é a saída? O problema é detectado na compilação, na execução ou nunca?
fn main() {
    let s = String::from("oi");
    let t = s;
    println!("{} {}", s, t);
}
```

### (a) Saída do Programa
```text
Compilação falhou com erro:
error[E0382]: borrow of moved value: `s`
 --> q5.rs:5:23
  |
3 |     let s = String::from("oi");
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
4 |     let t = s;
  |             - value moved here
5 |     println!("{} {}", s, t);
  |                       ^ value borrowed here after move
```

### (b) Quando o problema é detectado?
> **NA COMPILAÇÃO (Compile-time)**. O compilador `rustc` rejeita a construção do código antes que qualquer arquivo binário seja gerado.

### (c) Análise Conceitual e Fundamentação (Sebesta, Cap. 6)
1. **Gerenciamento de Memória Sem Coletor de Lixo:** Sebesta analisa em 6.11 que linguagens tradicionais enfrentam o dilema entre gerenciamento manual inseguro (ponteiros soltos em C) e gerenciamento automático pesado (coleta de lixo em Java e Go). Rust resolve essa equação histórica em tempo de compilação por meio do sistema de **Posse (*Ownership*)** e **Semântica de Movimento (*Move Semantics*)**.
2. **Mecanismo de Movimento:**
   * O tipo `String` gerencia um buffer alocado dinamicamente no heap, acompanhado de um ponteiro, capacidade e comprimento na pilha. Tipos que gerenciam memória no heap não implementam a trait `Copy`.
   * Na sentença `let t = s;`, o Rust não duplica o texto no heap (o que seria uma cópia profunda dispendiosa) e também não cria um ponteiro compartilhado tradicional (o que causaria o risco de *double free* ao final do escopo).
   * Em vez disso, o compilador **move a posse** exclusiva do recurso para a variável `t`, invalidando o identificador `s` a partir daquele ponto.
3. **Intercepção pelo Borrow Checker:**
   * Quando `println!` tenta acessar `s`, o *Borrow Checker* constata que foi tentado um empréstimo (*borrow*) de uma variável cujo valor já foi transferido.
   * O erro `E0382` é emitido em tempo de compilação.
4. **Vantagem em Engenharia de Software:** O bug é erradicado no ambiente do desenvolvedor, eliminando por completo vazamentos de memória, condições de corrida em concorrência e falhas de ponteiro solto sem adicionar um único ciclo de sobrecarga em tempo de execução.

---

## Questão 6 — C (Uniões Livres e Corrupção da Tipagem Estática)

### Código Original (`q6.c`)
```c
/* Exercício Aula 06 - Questão 6
 * ANTES de executar: qual é a saída? O problema é detectado na compilação, na execução ou nunca?
 */
#include <stdio.h>

int main(void) {
    union { int i; float f; } u;
    u.f = 1.0f;
    printf("%d\n", u.i);
    return 0;
}
```

### (a) Saída do Programa
```text
1065353216
```

### (b) Quando o problema é detectado?
> **NUNCA**. Nem o compilador C nem o sistema operacional detectam qualquer irregularidade. O programa executa e imprime a reinterpretação binária dos bits como se fosse um comportamento perfeitamente normal.

### (c) Análise Conceitual e Fundamentação (Sebesta, Cap. 6)
1. **Uniões Livres (*Free Unions*) vs. Discriminadas (Sebesta 6.10):**
   * Uma união é um tipo composto cujas variáveis compartilham exatamente o mesmo endereço físico de memória.
   * O C implementa **uniões livres**, o que significa que a linguagem não mantém nenhum discriminador (rótulo ou indicador de tipo ativo) e não impõe verificação estática ou dinâmica sobre qual membro foi escrito por último.
2. **Reinterpretação Binária em Memória:**
   * Ao atribuir `u.f = 1.0f;`, a máquina grava no endereço de `u` o padrão binário do número de ponto flutuante de precisão simples de 32 bits (IEEE 754):
     * Sinal ($1\text{ bit}$): $0$ (positivo)
     * Expoente polarizado ($8\text{ bits}$): $127 = 01111111_2$
     * Mantissa fracionária ($23\text{ bits}$): $00000000000000000000000_2$
     * Padrão binário completo: `0011 1111 1000 0000 0000 0000 0000 0000`
     * Representação hexadecimal: `0x3F800000`
   * Ao executar `printf("%d\n", u.i);`, o processador simplesmente lê esses mesmos 32 bits brutos sob a semântica de um número inteiro com sinal (`int` em complemento de dois).
   * O valor decimal correspondente a `0x3F800000` é rigorosamente **1065353216**.
3. **Quebra da Tipagem Estática (*Type Safety*):** Sebesta cita as uniões livres do C e C++ como o principal exemplo de quebra da segurança de tipos: elas permitem que um dado de determinado tipo seja lido arbitrariamente como outro, gerando buracos na blindagem de tipos da linguagem e abrindo precedentes para vazamento de informações ou ataques de corrupção de memória. Em contrapartida, linguagens modernas como Rust (com `enum` discriminado) e TypeScript (com tipos de união discriminada) exigem o tratamento seguro de cada variante.

---

## Síntese Comparativa: Análise de Trade-off no Projeto de Linguagens

Ordenação das abordagens quanto ao momento em que inconsistências de dados e memória são capturadas:

| Momento da Detecção | Linguagens / Exemplos | Vantagens | Desvantagens / Custos |
|---|---|---|---|
| **1. Na Compilação** (*Mais Cedo*) | **Rust** (`q5.rs` - erro E0382) | • Erros eliminados antes de ir para produção.<br>• Zero custo de verificação em runtime.<br>• Alta confiabilidade e segurança de memória. | • Maior complexidade sintática e curva de aprendizado.<br>• Tempo de compilação mais elevado. |
| **2. Na Execução** (*Intermediário*) | **Java** (`Q4.java` - `ArrayIndexOutOfBoundsException`) | • Previne acessos a dados corrompidos.<br>• Falhas são contidas em exceções tratáveis.<br>• Facilidade de depuração em runtime. | • Custo de ciclos de CPU para verificar limites a cada acesso.<br>• Risco de falha em produção caso a exceção não seja tratada. |
| **3. Nunca Detectado** (*Mais Tardio*) | **C** (`q6.c` - união livre)<br>**Go** (`q3.go` - estouro modular)<br>**JavaScript** (`q1.js` - IEEE 754) | • Máximo desempenho de máquina e controle direto de bits (C).<br>• Simplicidade de implementação do compilador (Go/JS). | • Vulnerabilidades de segurança (CWE).<br>• Corrupção silenciosa de regras de negócio financeiras/científicas. |

---

## Atividade Especial de Autopesquisa (Slide 23): Tipos de Dados Avançados

Conforme indicado nas anotações do Slide 23 pelo professor Munif e fundamentado em Sebesta (§6.6 a §6.9, p. 259–270), apresentamos a matriz comparativa sobre como as linguagens tratam **Matrizes Associativas**, **Registros** e **Tuplas**:

| Linguagem | Matriz Associativa (Dicionário/Mapa) | Registros / Estruturas | Tuplas |
|---|---|---|---|
| **Java** | `Map<K,V>` (`HashMap`, `TreeMap`).<br>• Chave e valor são referências a objetos.<br>• Ordem indefinida em `HashMap`, mas previsível em `LinkedHashMap`. | `record` (Java 16+).<br>• Registro imutável conciso.<br>• Gera automaticamente construtor, `equals()`, `hashCode()` e `toString()`. | Sem suporte nativo primitivo. Representado por records personalizados de 2 campos ou classes utilitárias de terceiros. |
| **Python** | `dict` (`{"chave": valor}`).<br>• Altamente otimizado em C.<br>• Mantém a **ordem de inserção garantida** desde o Python 3.7. | `class`, `@dataclass` ou `NamedTuple`.<br>• Mutável por padrão (exceto `dataclass(frozen=True)`). | `tuple` (`(1, "a", True)`).<br>• Coleção ordenada **imutável** de tamanho fixo.<br>• Suporta desempacotamento nativo. |
| **Go** | `map[K]V`.<br>• A ordem de iteração é **deliberadamente randomizada** pela runtime a cada execução para impedir que o desenvolvedor dependa da ordem interna. | `struct`.<br>• Composição de campos nomeados.<br>• Alocação contígua e controle de alinhamento de memória. | Sem tipo de tupla direto. Retorno múltiplo de funções simula tuplas para desempacotamento. |
| **Rust** | `HashMap<K, V>` (módulo `std::collections`).<br>• Usa por padrão o algoritmo resistente a ataques DoS (*SipHash*). | `struct` (C-like ou tuple structs).<br>• Controle estrito de layout e tempo de vida de memória.<br>• Imutável por padrão. | `(T1, T2, ...)` primitiva.<br>• Heterogênea, de tamanho fixo e alocada na pilha.<br>• Suporta casamento de padrões (*pattern matching*). |
