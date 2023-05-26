import { StyleSheet, View, Text } from 'react-native';
import FeatherIcon from '@expo/vector-icons/Feather';
import { ComponentProps } from 'react';

const styles = StyleSheet.create({
	container: {
		position: 'absolute',
		left: 18,
		right: 18,
		bottom: 24,
		backgroundColor: '#151517',
		borderRadius: 18,
		paddingVertical: 12,
		paddingHorizontal: 18,
	},
	action: {
		width: '100%',
		display: 'flex',
		flexDirection: 'row',
		alignItems: 'center',
		columnGap: 16,
		padding: 12,
	},
	actionLabel: {
		color: '#fff',
		fontSize: 24,
	},
});

type Action = {
	icon: ComponentProps<typeof FeatherIcon>['name'];
	label: string;
};

const actions: Array<Action> = [
	{
		icon: 'tag',
		label: 'Tags',
	},
	{
		icon: 'corner-up-right',
		label: 'Play Next',
	},
	{
		icon: 'plus-square',
		label: 'Add to Queue',
	},
	{
		icon: 'link',
		label: 'Share',
	},
	{
		icon: 'download',
		label: 'Download',
	},
	{
		icon: 'trash',
		label: 'Delete',
	},
];

export const MediaActions = () => {
	return (
		<View style={styles.container}>
			{actions.map(({ icon, label }, i) => (
				<View key={`${label}-${i}`} style={styles.action}>
					<FeatherIcon name={icon} size={30} color='#fff' />
					<Text style={styles.actionLabel}>{label}</Text>
				</View>
			))}
		</View>
	);
};
