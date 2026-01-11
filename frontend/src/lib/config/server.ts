import { COGNITO_USER_POOL_ID, PRIVATE_SENTRY_DSN } from '$env/static/private';

export const serverEnv = {
  cognitoUserPoolId: COGNITO_USER_POOL_ID,
  sentryDsn: PRIVATE_SENTRY_DSN || ''
};
