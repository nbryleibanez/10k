package main

import (
	"context"
	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
)

func loginHandler(ctx context.Context, event events.APIGatewayProxyRequest)
(events.APIGatewayProxyResponse, error) {
	return events.APIGatewayProxyResponse{
		StatusCode: 200,
		Body: "Hello from Lambda!"
	}, nil
}

func main() {
	lambda.Start(loginHandler)
}