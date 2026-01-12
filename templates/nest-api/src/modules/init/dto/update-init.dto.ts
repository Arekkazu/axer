import { PartialType } from '@nestjs/mapped-types';
import { CreateInitDto } from './create-init.dto';

export class UpdateInitDto extends PartialType(CreateInitDto) {}
