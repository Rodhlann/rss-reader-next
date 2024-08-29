import NextAuth, { Account, Session } from "next-auth"
import { JWT } from "next-auth/jwt"
import GithubProvider from "next-auth/providers/github"

export const authOptions = {
  providers: [
    GithubProvider({
      clientId: process.env.GITHUB_ID || '',
      clientSecret: process.env.GITHUB_SECRET || '',
    }),
  ],
  jwt: {
    encryption: true,
    secret: process.env.NEXTAUTH_SECRET
  },
  callbacks: {
    async session({ token, session }: { token: JWT, session: Session }) {
      session.accessToken = token.accessToken
      return session
    },
    async jwt({ token, account }: { token: JWT, account: Account | null }) {
      // Account only has access_token on initial load, and must be stored
      if (account) {
        token.accessToken = account?.access_token
      }
      return token
    },
  },
}

export default NextAuth(authOptions)