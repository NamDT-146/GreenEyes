import { Button, View, Text } from "react-native-ui-lib";
import QRScanner from "./scan";
import 'react-native-get-random-values'
import { StyleSheet } from "react-native";
import React, { useEffect, useState } from 'react'
import { decryptPayload } from "@/utils/decryptPayload";
import { PublicKey } from "@solana/web3.js";
import nacl from "tweetnacl";
import * as Linking from "expo-linking";
import bs58 from "bs58";

export default function connected() {
    const [phantomWalletPublicKey, setPhantomWalletPublicKey] =
        useState<PublicKey | null>(null);
    const [deeplink, setDeepLink] = useState<string>("");
    const [dappKeyPair] = useState(nacl.box.keyPair());
    const [sharedSecret, setSharedSecret] = useState<Uint8Array>();
    const [session, setSession] = useState<string>();
    useEffect(() => {
        const initializeDeeplinks = async () => {
            const initialUrl = await Linking.getInitialURL();
            if (initialUrl) {
                setDeepLink(initialUrl);
            }
        };
        initializeDeeplinks();
        const listener = Linking.addEventListener("url", (({ url }: Linking.EventType) => setDeepLink(url)));
        return () => {
            listener.remove();
        };
    }, []);
    useEffect(() => {
        if (!deeplink) return;

        const url = new URL(deeplink);
        const params = url.searchParams;

        // Handle an error response from Phantom
        if (params.get("errorCode")) {
            const error = Object.fromEntries([...params]);
            const message =
                error?.errorMessage ??
                JSON.stringify(Object.fromEntries([...params]), null, 2);
            console.log("error: ", message);
            return;
        }


        const sharedSecretDapp = nacl.box.before(
            bs58.decode(params.get("phantom_encryption_public_key")!),
            dappKeyPair.secretKey
        );
        const connectData = decryptPayload(
            params.get("data")!,
            params.get("nonce")!,
            sharedSecretDapp
        );
        setSharedSecret(sharedSecretDapp);
        setSession(connectData.session);
        setPhantomWalletPublicKey(new PublicKey(connectData.public_key));
        console.log(`connected to ${connectData.public_key.toString()}`);


    }, [deeplink]);
    return (
        <View>
            {phantomWalletPublicKey ? (
                <>
                    <View style={[styles.row, styles.wallet]}>
                        <View style={styles.greenDot} />
                        <Text
                            style={styles.text}
                            numberOfLines={1}
                            ellipsizeMode="middle"
                        >
                            {`Connected to: ${phantomWalletPublicKey.toString()}`}
                        </Text>
                    </View>
                    <View style={styles.row}>

                    </View>
                </>
            ) : (
                <View style={{ marginTop: 15 }}>
                    <Text>Not connected</Text>
                </View>
            )}
        </View>
    )
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: 'darkgrey',
        flexGrow: 1,
        position: "relative",
    },
    greenDot: {
        height: 8,
        width: 8,
        borderRadius: 10,
        marginRight: 5,
        backgroundColor: 'green',
    },
    header: {
        width: "95%",
        marginLeft: "auto",
        marginRight: "auto",
    },
    row: {
        flexDirection: "row",
        justifyContent: "space-between",
        marginBottom: 5,
    },
    spinner: {
        position: "absolute",
        alignSelf: "center",
        top: "50%",
        zIndex: 1000,
    },
    text: {
        color: 'lightgrey',
        width: "100%",
    },
    wallet: {
        alignItems: "center",
        margin: 10,
        marginBottom: 15,
    },
});