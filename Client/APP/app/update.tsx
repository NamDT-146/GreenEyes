import React, { useState } from 'react';
import { View, Text, TextInput, Button, Image, StyleSheet, TouchableOpacity } from 'react-native';
import * as ImagePicker from 'expo-image-picker';
import { useLocalSearchParams, useRouter } from 'expo-router';
import { Picker } from 'react-native-ui-lib';

const Send: React.FC = () => {
  const { productId } = useLocalSearchParams();
  const [productImage, setProductImage] = useState<string | null>(null);
  const [senderWalletId, setSenderWalletId] = useState<string>('');
  const [receiverWalletId, setReceiverWalletId] = useState<string>('');
  const router = useRouter();
  const [statusPicker, setStatusPicker] = useState<string>('update');

  const pickImage = async () => {
    const result = await ImagePicker.launchImageLibraryAsync({
      mediaTypes: ImagePicker.MediaTypeOptions.Images,
      allowsEditing: true,
      aspect: [4, 3],
      quality: 1,
    });

    if (!result.canceled) {
      setProductImage(result.assets[0].uri);
    }
  };

  const handleSubmit = () => {
    // Handle form submission logic here
    console.log({
      productImage,
      productId,
      senderWalletId,
      receiverWalletId,
    });
    router.push({
      pathname: '/info',
      params: {
        productImage,
        productId,
        senderWalletId,
        receiverWalletId,
      },
    });
  };

  return (
    <View style={styles.container}>
      <Text style={{
        fontSize: 20,
      }}>Product ID</Text>
      <TextInput
        placeholder="4de38fee-a4c6-439b-86cc-07a5c24fd1f2"
        value={productId as string}
        style={styles.input}
        editable={false}
      />
      <Text style={{
        fontSize: 20,
      }}>Status</Text>
      <Picker
        value={statusPicker}
        placeholder={'click to select a status'}
        onChange={((e) => console.log(setStatusPicker(e as string)))}
      >
        {['grounded', 'harvest', 'transfer', 'received', 'sold'].map(item => {
          return <Picker.Item value={item} label={item} />;
        })}

      </Picker>
      {statusPicker === 'transfer' &&
        <View>
          <Text style={{
            fontSize: 20,
          }}>Receiver address</Text>
          <TextInput
            placeholder="input wallet address"
            value={receiverWalletId}
            onChangeText={setReceiverWalletId}
            style={styles.input}
          />
        </View>
      }

      <TouchableOpacity onPress={pickImage} style={styles.imagePicker}>
        {productImage ? (
          <Image source={{ uri: productImage }} style={styles.image} />
        ) : (
          <Text>Select Image</Text>
        )}
      </TouchableOpacity>

      <Button title="Take action" onPress={handleSubmit} />
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    padding: 20,
    backgroundColor: '#fff',
  },
  imagePicker: {
    alignItems: 'center',
    justifyContent: 'center',
    height: 200,
    backgroundColor: '#eee',
    marginBottom: 20,
  },
  image: {
    width: '100%',
    height: '100%',
  },
  input: {
    height: 40,
    borderColor: '#ccc',
    borderWidth: 1,
    marginBottom: 20,
    paddingHorizontal: 10,
  },
});

export default Send;
