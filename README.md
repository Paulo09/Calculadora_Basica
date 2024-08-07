

                                                                    Calculadora - Rust


Crie uma estrutura de dados chamada "Operation" que contenha quatro variantes: "Addition", "Subtraction", "Multiplication" e "Division", cada um contendo dois campos do tipo "i32".
Escreva uma função chamada "calculate" que recebe uma instância de "Operation" como entrada e retorna um "Result" com um valor "i32" ou uma mensagem de erro "&'static str". A função deve usar um "match" para determinar qual operação está sendo solicitada e realizá-la. Se a operação for divisão e o divisor for zero, a função deve retornar "Err("Cannot divide by zero.")".
Teste sua função criando algumas instâncias de "Operation" e passando-as para a função "calculate". Certifique-se de tratar o caso de divisão por zero.
Dicas:
Lembre-se de usar a cláusula "if" dentro do "match" para verificar se o divisor é zero antes de realizar a divisão.
Use o método "unwrap()" com cuidado, pois ele pode causar panics se a operação retornar um erro. É melhor usar "match" para tratar casos de erro.
Perguntas dessa tarefa
Crie o arquivo operations.rs

crie o arquivo main.rs


Mostrar rascunhos


Criando o Arquivo operations.rs
Rust
enum Operation {
    Addition(i32, i32),
    Subtraction(i32, i32),
    Multiplication(i32, i32),
    Division(i32, i32),
}

fn calculate(operation: Operation) -> Result<i32, &'static str> {
    match operation {
        Operation::Addition(a, b) => Ok(a + b),
        Operation::Subtraction(a, b) => Ok(a - b),
        Operation::Multiplication(a, b) => Ok(a * b),
        Operation::Division(a, b) => {
            if b == 0 {
                Err("Cannot divide by zero")
            } else {
                Ok(a / b)
            }
        }
    }
}
Use o código com cuidado.

Criando o Arquivo main.rs
Rust
mod operations;

fn main() {
    let addition = operations::Operation::Addition(5, 3);
    let subtraction = operations::Operation::Subtraction(10, 4);
    let multiplication = operations::Operation::Multiplication(2, 6);
    let division = operations::Operation::Division(12, 3);
    let division_by_zero = operations::Operation::Division(10, 0);

    println!("Addition: {}", calculate(addition).unwrap());
    println!("Subtraction: {}", calculate(subtraction).unwrap());
    println!("Multiplication: {}", calculate(multiplication).unwrap());
    println!("Division: {}", calculate(division).unwrap());

    match calculate(division_by_zero) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}
Use o código com cuidado.

Explicação:
operations.rs
Enum Operation: Define as quatro operações possíveis, cada uma com dois inteiros como operandos.
Função calculate:
Recebe uma instância de Operation.
Utiliza match para desconstruir a enum e realizar a operação correspondente.
Verifica a divisão por zero no caso da operação de divisão.
Retorna Result<i32, &'static str> para indicar sucesso (com o resultado) ou falha (com uma mensagem de erro).
main.rs
mod operations;: Importa o módulo operations.
Criação de instâncias: Cria várias instâncias de Operation para testar as diferentes operações.
Chamadas a calculate: Chama a função calculate para cada operação e imprime o resultado ou a mensagem de erro.
Tratamento de erro: Demonstra como usar match para tratar o caso de divisão por zero.
Pontos importantes:

Enum: A utilização de enum permite representar diferentes tipos de dados dentro de uma única variável.
Match: O match é ideal para desconstruir enums e realizar ações específicas para cada variante.
Result: O Result é usado para representar operações que podem falhar, fornecendo um valor em caso de sucesso ou um erro em caso de falha.
Tratamento de erros: O código demonstra como lidar com erros de forma segura, evitando panic!.
Melhorias:

Personalização de mensagens de erro: As mensagens de erro podem ser mais específicas e informativas.
Tipos numéricos: Pode-se utilizar outros tipos numéricos como f64 para operações de ponto flutuante.
Mais operações: É possível adicionar outras operações matemáticas.
Generics: Para tornar a função calculate mais genérica, pode-se utilizar generics para permitir diferentes tipos numéricos.
Este código fornece uma base sólida para realizar operações matemáticas em Rust de forma segura e concisa.
