import { StyleSheet, View, Text } from 'react-native';

const styles = StyleSheet.create({
	container: {
		flex: 1,
		backgroundColor: '#fff',
		alignItems: 'center',
		justifyContent: 'center',
	},
});

export const App = () => (
	<View style={styles.container}>
		<Text>Hello world</Text>
	</View>
);
