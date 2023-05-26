import Feather from '@expo/vector-icons/Feather';
import { SafeAreaView, StyleSheet, Text, View } from 'react-native';

const styles = StyleSheet.create({
	background: {
		backgroundColor: '#832b83',
	},
	container: {
		paddingTop: 8,
		paddingLeft: 16,
		paddingBottom: 12,
		paddingRight: 24,
		width: '100%',
		display: 'flex',
		flexDirection: 'row',
		justifyContent: 'space-between',
		alignItems: 'center',
	},
	title: {
		color: '#fff',
		fontWeight: '700',
		fontSize: 48,
		flexShrink: 1,
	},
	actionsContainer: {
		display: 'flex',
		flexDirection: 'row',
		columnGap: 24,
	},
});

type PageHeaderProps = {
	title: string;
};

export const PageHeader = ({ title }: PageHeaderProps) => {
	return (
		<SafeAreaView style={styles.background}>
			<View style={styles.container}>
				<Text style={styles.title}>{title}</Text>
				<View style={styles.actionsContainer}>
					<Feather name='plus-circle' size={32} color='#fff' />
					<Feather name='search' size={32} color='#fff' />
				</View>
			</View>
		</SafeAreaView>
	);
};
