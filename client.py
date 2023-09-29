import socket        # Import socket module
import binascii
import time

clientsocket = socket.socket()         # Create a socket object
clientsocket.connect(('127.0.0.1', 7878))

packagetosend = binascii.unhexlify("78780d010358735070477194007edf520d0a")


print(clientsocket.send(packagetosend))

clientsocket.close()
