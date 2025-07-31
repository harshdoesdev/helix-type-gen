// Generated TypeScript types for HelixDB schema
// This file is auto-generated. Do not edit manually.
// Generated with helix-ts-gen from introspection endpoint

import HelixDB from 'helix-ts';

// Utility types
export type Vector = number[];
export type Optional<T> = T | null | undefined;
export type HelixID = bigint;

export interface BaseNode {
  id: HelixID;
  created_at?: number;
  updated_at?: number;
}

export interface BaseVector {
  id: HelixID;
}

export interface BaseEdge {
  id?: HelixID;
  created_at?: number;
}

// HelixDB types (extracted from runtime since not exported)
export type HelixDBClient = HelixDB;
export type HelixDBResponse = Awaited<ReturnType<HelixDB['query']>>;
export type HelixDBInput = Parameters<HelixDB['query']>[1];

// Node types
export interface User extends BaseNode {
  id: bigint;
  is_admin: boolean;
  name: string;
  label: string;
  age: number;
}

export type AnyNode = User;

// Vector types
// Edge types
export interface Knows extends BaseEdge {
  from: User;
  to: User;
  since: bigint;
}

export type AnyEdge = Knows;

// Query parameter types
// Note: Return types are not generated as they cannot be inferred from schema introspection

export interface GetUserFriendsParams {
  user_id: bigint;
}

// Client interface for type-safe HelixDB operations
export interface TypedHelixDBClient {
  GetUserFriends(params: GetUserFriendsParams): Promise<HelixDBResponse>;
}

// SDK Integration Helpers
/**
 * Create a type-safe HelixDB client wrapper
 * Usage:
 *   const client = new HelixDB();
 *   const typedClient = createTypedClient(client);
 */
export function createTypedClient(client: HelixDB): TypedHelixDBClient {
  return {
    async GetUserFriends(params) {
      return await client.query('GetUserFriends', params);
    },
  };
}

// Query endpoint constants
export const QueryEndpoints = {
  GET_USER_FRIENDS: 'GetUserFriends',
} as const;

// Type guards and validation helpers
export function isUser(obj: any): obj is User {
  return obj && typeof obj === 'object' && typeof obj.id === 'bigint';
}

/**
 * Helper to extract data from HelixDB response with basic type assertion
 * Usage: const users = extractData<User[]>(response, 'users');
 */
export function extractData<T>(response: HelixDBResponse, key?: string): T {
  if (key) {
    return response[key] as T;
  }
  return response as T;
}

