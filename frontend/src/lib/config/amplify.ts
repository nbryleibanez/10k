import { Amplify } from '@aws-amplify/core';
import { clientEnv } from '$config/env';

let configured = false;

export function configureAmplify() {
  if (configured || clientEnv.mockAuth) {
    return;
  }
  Amplify.configure({
    Auth: {
      Cognito: {
        userPoolId: clientEnv.cognitoUserPoolId,
        userPoolClientId: clientEnv.cognitoClientId,
        region: clientEnv.cognitoRegion
      }
    }
  });
  configured = true;
}
