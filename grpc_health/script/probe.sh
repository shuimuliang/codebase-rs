# https://github.com/grpc-ecosystem/grpc-health-probe

while [ true ]; do
./grpc_health_probe -addr=[::1]:50051 -service=helloworld.Greeter
sleep 1
done

#console output:
#status: SERVING
#service unhealthy (responded with "NOT_SERVING")
#status: SERVING
#service unhealthy (responded with "NOT_SERVING")
