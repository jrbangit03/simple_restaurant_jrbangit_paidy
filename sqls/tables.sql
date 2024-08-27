CREATE TABLE `order` (
     `order_id` int NOT NULL AUTO_INCREMENT,
     `table_number` int NOT NULL,
     `food_id` int NOT NULL,
     `food_name` varchar(255) NOT NULL,
     `quantity` int NOT NULL,
     PRIMARY KEY (`order_id`)
);