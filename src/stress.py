from concurrent.futures import ThreadPoolExecutor
import sys
import os

"""
    Función para procesar los comandos ingresados por el usuario, distingue el número de threads, host y
    la lista de comandos a ejecutar
"""
def process_command(argumentos):
    len_args = len(argumentos)
    
    if len_args < 7:
        print("Error sintaxis: stress -n <cantidad-hilos> HTTPclient -h <host> [<lista-comandos-a-ejecutar>]")
        sys.exit()
    
    client = ['HTTPClient']

    for i in range(1, len_args):
        if (argumentos[i] == "-n"):
            threads = int(argumentos[i+1])
        elif (argumentos[i] == "-h"):
            client.extend(argumentos[i+1:])

    return [threads, client]


"""
    Convierte una lista a un string
"""
def listToString(command_list): 
    str = "" 
    for i in command_list: 
        str += i + " "
    return str 


"""
    Utiliza ThreadPoolExecutor para ejecutar una llamada las veces que el usuario
    haya especificado como parametro 
"""
def main():
    argumentos = sys.argv
    (num_threads, lista_comando_cliente) = process_command(argumentos)
    comando_cliente = listToString(lista_comando_cliente)
    print("Numero de Threads: " + str(num_threads))
    print("Comando HTTPClient: " + comando_cliente)
    
    with ThreadPoolExecutor() as executor:
        for n in range(num_threads):
            executor.submit(os.system("cargo run --bin " + comando_cliente))

main()

