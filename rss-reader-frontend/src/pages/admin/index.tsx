import React from "react";
import { signIn, signOut, useSession } from "next-auth/react";
import Feeds from "@/components/feeds";

export default function Admin() {
  const { data: session } = useSession()
 
  if (session) {
    return (
      <>
        Signed in as {session?.user?.email} <br />
        <button onClick={() => signOut()}>Sign out</button>

        <Feeds />
      </>
    )
  }
  return (
    <>
      Not signed in <br />
      <button onClick={() => signIn()}>Sign in</button>
    </>
  )
}