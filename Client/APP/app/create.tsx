import React, { useState } from 'react';
import { View, TextInput, StyleSheet } from 'react-native';
import { Text, Button, Picker, Incubator } from 'react-native-ui-lib';

const CreateProduct = () => {
  const [foodType, setFoodType] = useState('');
  const [location, setLocation] = useState('');

  const foodTypes = ['Fruit', 'Vegetable', 'Meat', 'Dairy'];
  const locations = ['New York', 'California', 'Texas', 'Florida'];

  const handleSubmit = () => {
    // Handle form submission
    const productData = { foodType, location };
    console.log('Product data:', productData);
    // Add form submission logic, e.g., sending data to an API
  };

  return (
    <View style={styles.container}>
      <Text text50 marginB-20>Create Product</Text>

      {/* Food Type Picker */}
      <Picker
        placeholder="Select Food Type"
        value={foodType}
        showSearch
        onChange={(item) => setFoodType(item as string)}
        marginB-20
      >
        {foodTypes.map((type) => (
          <Picker.Item key={type} value={type} label={type} />
        ))}
      </Picker>

      {/* Location Picker */}
      <Picker
        placeholder="Select Location"
        value={location}
        onChange={(item) => setLocation(item as string)}
        marginB-20
      >
        {locations.map((loc) => (
          <Picker.Item key={loc} value={loc} label={loc} />
        ))}
      </Picker>

      {/* Submit Button */}
      <Button
        label="Create Product"
        onPress={handleSubmit}
        backgroundColor="#6200EE"
        marginT-20
      />
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    padding: 20,
    justifyContent: 'center',
  },
});

export default CreateProduct;
