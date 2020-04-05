import Sequelize from 'sequelize';

const sequelize = new Sequelize(
  'd506dmpebgtn40', 'pklvlbabebheru', '3b809317c45d1068a6e1140d06856d367cf7eab3130de9e330250e1b5d4a82ba', {
    host: 'ec2-34-206-252-187.compute-1.amazonaws.com',
    dialect: 'postgres',
    dialectOptions: {
      ssl: {
        rejectUnauthorized: false
      }
   }
 }
);

const models = {
  User: sequelize.import('./user')
};

Object.keys(models).forEach(key => {
  if ('associate' in models[key]) {
    models[key].associate(models);
  }
});

export { sequelize };
export default models;