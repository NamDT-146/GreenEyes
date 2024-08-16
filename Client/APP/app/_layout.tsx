import { Stack } from "expo-router";

export default function RootLayout() {
  return (
    <Stack>
      <Stack.Screen name="index" options={{ headerShown: false}} />
      <Stack.Screen name="home" options={{ headerShown: false }} />
      <Stack.Screen name="info" options={{ headerShown: true }} />
      <Stack.Screen name="update" options={{ headerShown: true, title: "Update (prototype, not complete)" }} />
    </Stack>
  );
}
