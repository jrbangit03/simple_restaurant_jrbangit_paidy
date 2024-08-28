CREATE TABLE `order` (
     `order_id` int NOT NULL AUTO_INCREMENT,
     `table_number` int NOT NULL,
     `food_id` int NOT NULL,
     `food_name` varchar(255) NOT NULL,
     `quantity` int NOT NULL,
     PRIMARY KEY (`order_id`)
);

CREATE TABLE `food` (
     `food_id` int NOT NULL AUTO_INCREMENT,
     `food_name` varchar(255) NOT NULL,
     `cooking_time_in_mins` int NOT NULL,
     `in_stock` boolean NOT NULL,
     PRIMARY KEY (`food_id`)
);

CREATE TABLE `dine_in_tables` (
     `table_number` int NOT NULL,
     `table_status` varchar(255) NOT NULL,
     PRIMARY KEY (`table_number`)
);