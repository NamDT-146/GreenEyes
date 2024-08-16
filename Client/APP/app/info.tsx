import React, { useEffect, useState } from 'react';
import { FlatList, StyleSheet } from 'react-native';
import { router, useLocalSearchParams } from 'expo-router';
import axios from 'axios';
import { ListItem, View, Text, Image, Button } from 'react-native-ui-lib';
import timeToString from '@/utils/timeToString';


type ProductInfo = {
  type: 'info';
  seedType: string;
  productName: string;
  productImage: string;
  productId: string;
}

type StatusBlock = {
  type: 'status';
  time: number;
  status: string;
  location?: string;
  image: string;
  owner: string;
}

type ProductDataBlock = ProductInfo | StatusBlock;

const Info: React.FC = () => {
  const { productId } = useLocalSearchParams();
  const [productInfo, setProductInfo] = useState<any | null>(null);
  const [productStatusChain, setProductStatusChain] = useState<StatusBlock[]>([]);
  useEffect(() => {
    // Fetch product info from API
    axios.get(`https://api.mailtool.gdsc.vn/#api/products/${productId}`)
      .then(() => {
        const data: ProductDataBlock[] = [
          {
            type: 'info',
            seedType: 'carrot',
            productName: 'Carrot',
            productImage: 'https://upload.wikimedia.org/wikibooks/vi/9/97/Hay_Day_-_C%C3%A0_r%E1%BB%91t.png',
            productId: '4de38fee-a4c6-439b-86cc-07a5c24fd1f2',
          },
          {
            type: 'status',
            time: new Date('2024-07-01').getTime(),
            status: 'grounded',
            image: `https://mic.mediacdn.vn/Upload/Store/tintuc/vietnam/4/20131030b2.jpg`,
            owner: '6mVtKPkc2bNgFJCRFKbvpT9Utz9xAqgUtHULVSXMQik1'
          },
          {
            type: 'status',
            time: new Date('2024-08-01').getTime(),
            status: 'harvest',
            image: `https://cdnmedia.baotintuc.vn/Upload/c2tvplmdloSDblsn03qN2Q/files/2022/01/09/haiduong/cac-rot-hai-duong-912022-6.jpg`,
            owner: '6mVtKPkc2bNgFJCRFKbvpT9Utz9xAqgUtHULVSXMQik1'
          },
          {
            type: 'status',
            time: new Date('2024-08-03').getTime(),
            status: 'transfered',
            image: `https://paka.vn/wp-content/uploads/2023/01/ban-thung-carton-dong-hang-di-my-paka-593x800.jpg`,
            owner: '6mVtKPkc2bNgFJCRFKbvpT9Utz9xAqgUtHULVSXMQik1'
          },
          {
            type: 'status',
            time: new Date('2024-08-03').getTime(),
            status: 'received',
            image: `https://paka.vn/wp-content/uploads/2023/01/ban-thung-carton-dong-hang-di-my-paka-593x800.jpg`,
            owner: '1a2cKPkc2bNgFJCRFKbvpT9Utz9xAqgUtHULVSXM1asc'
          },
          {
            type: 'status',
            time: new Date('2024-08-03').getTime(),
            status: 'sold',
            image: `https://images-handler.kamereo.vn/eyJidWNrZXQiOiJpbWFnZS1oYW5kbGVyLXByb2QiLCJrZXkiOiJzdXBwbGllci82NTQvUFJPRFVDVF9JTUFHRS9kOGEyNjYxOC02NjI5LTQ3OTItOWVhNS1kODZhYjdiOTAxZjIucG5nIiwiZWRpdHMiOnsicmVzaXplIjp7IndpZHRoIjo1MDAsImhlaWdodCI6NTAwLCJmaXQiOiJmaWxsIn19fQ==`,
            owner: '1a2cKPkc2bNgFJCRFKbvpT9Utz9xAqgUtHULVSXM1asc'
          },
        ]
        return data;
      })
      .then((data) => {
        if (data.length > 0) {
          setProductInfo(data.shift())
        }
        setProductStatusChain(data as StatusBlock[]);
      })
      .catch((error) => console.error(error));
  }, []);

  if (!productId) {
    return (
      <View style={styles.center}>
        <Text>No product ID found</Text>
      </View>
    );
  }

  if (!productInfo) {
    return (
      <View style={styles.center}>
        <Text>Loading product info...</Text>
      </View>
    );
  }

  const onSendPress = () => {
    router.push({
      pathname: '/update',
      params: {
        productId: productInfo.productId,

      },
    })
  }


  return (
    <View style={{
      flex: 1
    }}>
      <View style={{
        flex: 1,
        minHeight: 25,
      }}>
        {/* <View style={{
          ...styles.center
          } }>
          {typeof productInfo.productImage == 'string' && <Image source={{ uri: productInfo.productImage }} style={styles.image} />}
        </View> */}
        <View style={{
          padding: 20
        }}>
          <Text>Product Type: {productInfo.productName}</Text>
          <Text>Product ID: {productInfo.productId}</Text>
        </View>
      </View>
      <View style={{
        display: 'flex',
        flexGrow: 1
      }}>
        <Text>Transfer log</Text>
        <FlatList
          data={productStatusChain}
          renderItem={({ item, index }) => {
            return (
              <ListItem onPress={() => console.log('pressed')}>
                <ListItem.Part left>
                  <Image source={{ uri: item.image }} style={styles.listItemImage} />
                </ListItem.Part>
                <ListItem.Part middle column containerStyle={[styles.border, { paddingRight: 17 }]}>
                  <ListItem.Part containerStyle={{ marginBottom: 3 }}>
                    <Text grey10 text70 style={{ flex: 1, marginRight: 10 }} numberOfLines={1}>
                      {item.status}
                    </Text>
                    <Text grey10 text70 style={{ marginTop: 2 }}>
                      {timeToString(item.time)}
                    </Text>
                  </ListItem.Part>
                  <ListItem.Part>
                    <Text
                      style={{ flex: 1, marginRight: 10 }}
                      text90
                      grey40
                      numberOfLines={1}
                    >{`${item.owner} item`}</Text>
                  </ListItem.Part>
                </ListItem.Part>
              </ListItem>
            )
          }}
          keyExtractor={(item, index) => index.toString()}
        />


      </View>
      <View style={{
        padding: 20,
        marginTop: 20
      }}>
        <Button onPress={onSendPress} >
          <Text>Update Status / Transfer</Text>
        </Button>
      </View>

    </View>
  );
};

const styles = StyleSheet.create({
  flex: {
    flex: 1
  },
  center: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  data: {
    marginTop: 20,
    fontSize: 16,
    fontWeight: 'bold',
  },
  image: {
    width: 50,
    height: 50,
    borderColor: 'grey',
    borderWidth: 1,
    borderRadius: 10,
  },
  border: {
    borderBottomWidth: StyleSheet.hairlineWidth,
    borderColor: 'grey',
  },
  listItemImage: {
    width: 54,
    height: 54,
    borderRadius: 20,
    marginHorizontal: 14
  },
});

export default Info;
