import {
  PUBLIC_API_BASE_URL,
  PUBLIC_COGNITO_CLIENT_ID,
  PUBLIC_COGNITO_REGION,
  PUBLIC_COGNITO_USER_POOL_ID,
  PUBLIC_ENABLE_MOCK_AUTH,
  PUBLIC_SENTRY_DSN
} from '$env/static/public';

export type ClientEnv = {
  apiBaseUrl: string;
  cognitoClientId: string;
  cognitoRegion: string;
  cognitoUserPoolId: string;
  sentryDsn?: string;
  mockAuth: boolean;
};

export const clientEnv: ClientEnv = {
  apiBaseUrl: PUBLIC_API_BASE_URL || '',
  cognitoClientId: PUBLIC_COGNITO_CLIENT_ID || 'local-client',
  cognitoRegion: PUBLIC_COGNITO_REGION || 'us-east-1',
  cognitoUserPoolId: PUBLIC_COGNITO_USER_POOL_ID || 'local-pool',
  sentryDsn: PUBLIC_SENTRY_DSN || undefined,
  mockAuth: PUBLIC_ENABLE_MOCK_AUTH === 'true'
};
