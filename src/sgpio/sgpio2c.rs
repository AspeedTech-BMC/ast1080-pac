#[doc = "Register `SGPIO2C` reader"]
pub type R = crate::R<Sgpio2cSpec>;
#[doc = "Register `SGPIO2C` writer"]
pub type W = crate::W<Sgpio2cSpec>;
#[doc = "Field `OutputValueOfSGPIO64` reader - Output value of SGPIO_64"]
pub type OutputValueOfSgpio64R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO65` reader - Output value of SGPIO_65"]
pub type OutputValueOfSgpio65R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO66` reader - Output value of SGPIO_66"]
pub type OutputValueOfSgpio66R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO67` reader - Output value of SGPIO_67"]
pub type OutputValueOfSgpio67R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO68` reader - Output value of SGPIO_68"]
pub type OutputValueOfSgpio68R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO69` reader - Output value of SGPIO_69"]
pub type OutputValueOfSgpio69R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO70` reader - Output value of SGPIO_70"]
pub type OutputValueOfSgpio70R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO71` reader - Output value of SGPIO_71"]
pub type OutputValueOfSgpio71R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO72` reader - Output value of SGPIO_72"]
pub type OutputValueOfSgpio72R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO73` reader - Output value of SGPIO_73"]
pub type OutputValueOfSgpio73R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO74` reader - Output value of SGPIO_74"]
pub type OutputValueOfSgpio74R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO75` reader - Output value of SGPIO_75"]
pub type OutputValueOfSgpio75R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO76` reader - Output value of SGPIO_76"]
pub type OutputValueOfSgpio76R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO77` reader - Output value of SGPIO_77"]
pub type OutputValueOfSgpio77R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO78` reader - Output value of SGPIO_78"]
pub type OutputValueOfSgpio78R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO79` reader - Output value of SGPIO_79"]
pub type OutputValueOfSgpio79R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO80` reader - Output value of SGPIO_80"]
pub type OutputValueOfSgpio80R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO81` reader - Output value of SGPIO_81"]
pub type OutputValueOfSgpio81R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO82` reader - Output value of SGPIO_82"]
pub type OutputValueOfSgpio82R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO83` reader - Output value of SGPIO_83"]
pub type OutputValueOfSgpio83R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO84` reader - Output value of SGPIO_84"]
pub type OutputValueOfSgpio84R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO85` reader - Output value of SGPIO_85"]
pub type OutputValueOfSgpio85R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO86` reader - Output value of SGPIO_86"]
pub type OutputValueOfSgpio86R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO87` reader - Output value of SGPIO_87"]
pub type OutputValueOfSgpio87R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO88` reader - Output value of SGPIO_88"]
pub type OutputValueOfSgpio88R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO89` reader - Output value of SGPIO_89"]
pub type OutputValueOfSgpio89R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO90` reader - Output value of SGPIO_90"]
pub type OutputValueOfSgpio90R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO91` reader - Output value of SGPIO_91"]
pub type OutputValueOfSgpio91R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO92` reader - Output value of SGPIO_92"]
pub type OutputValueOfSgpio92R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO93` reader - Output value of SGPIO_93"]
pub type OutputValueOfSgpio93R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO94` reader - Output value of SGPIO_94"]
pub type OutputValueOfSgpio94R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO95` reader - Output value of SGPIO_95"]
pub type OutputValueOfSgpio95R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Output value of SGPIO_64"]
    #[inline(always)]
    pub fn output_value_of_sgpio64(&self) -> OutputValueOfSgpio64R {
        OutputValueOfSgpio64R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output value of SGPIO_65"]
    #[inline(always)]
    pub fn output_value_of_sgpio65(&self) -> OutputValueOfSgpio65R {
        OutputValueOfSgpio65R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output value of SGPIO_66"]
    #[inline(always)]
    pub fn output_value_of_sgpio66(&self) -> OutputValueOfSgpio66R {
        OutputValueOfSgpio66R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output value of SGPIO_67"]
    #[inline(always)]
    pub fn output_value_of_sgpio67(&self) -> OutputValueOfSgpio67R {
        OutputValueOfSgpio67R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output value of SGPIO_68"]
    #[inline(always)]
    pub fn output_value_of_sgpio68(&self) -> OutputValueOfSgpio68R {
        OutputValueOfSgpio68R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Output value of SGPIO_69"]
    #[inline(always)]
    pub fn output_value_of_sgpio69(&self) -> OutputValueOfSgpio69R {
        OutputValueOfSgpio69R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Output value of SGPIO_70"]
    #[inline(always)]
    pub fn output_value_of_sgpio70(&self) -> OutputValueOfSgpio70R {
        OutputValueOfSgpio70R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output value of SGPIO_71"]
    #[inline(always)]
    pub fn output_value_of_sgpio71(&self) -> OutputValueOfSgpio71R {
        OutputValueOfSgpio71R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output value of SGPIO_72"]
    #[inline(always)]
    pub fn output_value_of_sgpio72(&self) -> OutputValueOfSgpio72R {
        OutputValueOfSgpio72R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output value of SGPIO_73"]
    #[inline(always)]
    pub fn output_value_of_sgpio73(&self) -> OutputValueOfSgpio73R {
        OutputValueOfSgpio73R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output value of SGPIO_74"]
    #[inline(always)]
    pub fn output_value_of_sgpio74(&self) -> OutputValueOfSgpio74R {
        OutputValueOfSgpio74R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output value of SGPIO_75"]
    #[inline(always)]
    pub fn output_value_of_sgpio75(&self) -> OutputValueOfSgpio75R {
        OutputValueOfSgpio75R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output value of SGPIO_76"]
    #[inline(always)]
    pub fn output_value_of_sgpio76(&self) -> OutputValueOfSgpio76R {
        OutputValueOfSgpio76R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output value of SGPIO_77"]
    #[inline(always)]
    pub fn output_value_of_sgpio77(&self) -> OutputValueOfSgpio77R {
        OutputValueOfSgpio77R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output value of SGPIO_78"]
    #[inline(always)]
    pub fn output_value_of_sgpio78(&self) -> OutputValueOfSgpio78R {
        OutputValueOfSgpio78R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Output value of SGPIO_79"]
    #[inline(always)]
    pub fn output_value_of_sgpio79(&self) -> OutputValueOfSgpio79R {
        OutputValueOfSgpio79R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output value of SGPIO_80"]
    #[inline(always)]
    pub fn output_value_of_sgpio80(&self) -> OutputValueOfSgpio80R {
        OutputValueOfSgpio80R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Output value of SGPIO_81"]
    #[inline(always)]
    pub fn output_value_of_sgpio81(&self) -> OutputValueOfSgpio81R {
        OutputValueOfSgpio81R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output value of SGPIO_82"]
    #[inline(always)]
    pub fn output_value_of_sgpio82(&self) -> OutputValueOfSgpio82R {
        OutputValueOfSgpio82R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output value of SGPIO_83"]
    #[inline(always)]
    pub fn output_value_of_sgpio83(&self) -> OutputValueOfSgpio83R {
        OutputValueOfSgpio83R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output value of SGPIO_84"]
    #[inline(always)]
    pub fn output_value_of_sgpio84(&self) -> OutputValueOfSgpio84R {
        OutputValueOfSgpio84R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Output value of SGPIO_85"]
    #[inline(always)]
    pub fn output_value_of_sgpio85(&self) -> OutputValueOfSgpio85R {
        OutputValueOfSgpio85R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Output value of SGPIO_86"]
    #[inline(always)]
    pub fn output_value_of_sgpio86(&self) -> OutputValueOfSgpio86R {
        OutputValueOfSgpio86R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Output value of SGPIO_87"]
    #[inline(always)]
    pub fn output_value_of_sgpio87(&self) -> OutputValueOfSgpio87R {
        OutputValueOfSgpio87R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Output value of SGPIO_88"]
    #[inline(always)]
    pub fn output_value_of_sgpio88(&self) -> OutputValueOfSgpio88R {
        OutputValueOfSgpio88R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Output value of SGPIO_89"]
    #[inline(always)]
    pub fn output_value_of_sgpio89(&self) -> OutputValueOfSgpio89R {
        OutputValueOfSgpio89R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Output value of SGPIO_90"]
    #[inline(always)]
    pub fn output_value_of_sgpio90(&self) -> OutputValueOfSgpio90R {
        OutputValueOfSgpio90R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Output value of SGPIO_91"]
    #[inline(always)]
    pub fn output_value_of_sgpio91(&self) -> OutputValueOfSgpio91R {
        OutputValueOfSgpio91R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Output value of SGPIO_92"]
    #[inline(always)]
    pub fn output_value_of_sgpio92(&self) -> OutputValueOfSgpio92R {
        OutputValueOfSgpio92R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Output value of SGPIO_93"]
    #[inline(always)]
    pub fn output_value_of_sgpio93(&self) -> OutputValueOfSgpio93R {
        OutputValueOfSgpio93R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Output value of SGPIO_94"]
    #[inline(always)]
    pub fn output_value_of_sgpio94(&self) -> OutputValueOfSgpio94R {
        OutputValueOfSgpio94R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Output value of SGPIO_95"]
    #[inline(always)]
    pub fn output_value_of_sgpio95(&self) -> OutputValueOfSgpio95R {
        OutputValueOfSgpio95R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Debug Serial In Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio2c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio2c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio2cSpec;
impl crate::RegisterSpec for Sgpio2cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio2c::R`](R) reader structure"]
impl crate::Readable for Sgpio2cSpec {}
#[doc = "`write(|w| ..)` method takes [`sgpio2c::W`](W) writer structure"]
impl crate::Writable for Sgpio2cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO2C to value 0"]
impl crate::Resettable for Sgpio2cSpec {}
