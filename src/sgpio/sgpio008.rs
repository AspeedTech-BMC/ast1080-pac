#[doc = "Register `SGPIO008` reader"]
pub type R = crate::R<Sgpio008Spec>;
#[doc = "Register `SGPIO008` writer"]
pub type W = crate::W<Sgpio008Spec>;
#[doc = "Field `InputValueOfSGPIO32` reader - Input value of SGPIO_32"]
pub type InputValueOfSgpio32R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO33` reader - Input value of SGPIO_33"]
pub type InputValueOfSgpio33R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO34` reader - Input value of SGPIO_34"]
pub type InputValueOfSgpio34R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO35` reader - Input value of SGPIO_35"]
pub type InputValueOfSgpio35R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO36` reader - Input value of SGPIO_36"]
pub type InputValueOfSgpio36R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO37` reader - Input value of SGPIO_37"]
pub type InputValueOfSgpio37R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO38` reader - Input value of SGPIO_38"]
pub type InputValueOfSgpio38R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO39` reader - Input value of SGPIO_39"]
pub type InputValueOfSgpio39R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO40` reader - Input value of SGPIO_40"]
pub type InputValueOfSgpio40R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO41` reader - Input value of SGPIO_41"]
pub type InputValueOfSgpio41R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO42` reader - Input value of SGPIO_42"]
pub type InputValueOfSgpio42R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO43` reader - Input value of SGPIO_43"]
pub type InputValueOfSgpio43R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO44` reader - Input value of SGPIO_44"]
pub type InputValueOfSgpio44R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO45` reader - Input value of SGPIO_45"]
pub type InputValueOfSgpio45R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO46` reader - Input value of SGPIO_46"]
pub type InputValueOfSgpio46R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO47` reader - Input value of SGPIO_47"]
pub type InputValueOfSgpio47R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO48` reader - Input value of SGPIO_48"]
pub type InputValueOfSgpio48R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO49` reader - Input value of SGPIO_49"]
pub type InputValueOfSgpio49R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO50` reader - Input value of SGPIO_50"]
pub type InputValueOfSgpio50R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO51` reader - Input value of SGPIO_51"]
pub type InputValueOfSgpio51R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO52` reader - Input value of SGPIO_52"]
pub type InputValueOfSgpio52R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO53` reader - Input value of SGPIO_53"]
pub type InputValueOfSgpio53R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO54` reader - Input value of SGPIO_54"]
pub type InputValueOfSgpio54R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO55` reader - Input value of SGPIO_55"]
pub type InputValueOfSgpio55R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO56` reader - Input value of SGPIO_56"]
pub type InputValueOfSgpio56R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO57` reader - Input value of SGPIO_57"]
pub type InputValueOfSgpio57R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO58` reader - Input value of SGPIO_58"]
pub type InputValueOfSgpio58R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO59` reader - Input value of SGPIO_59"]
pub type InputValueOfSgpio59R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO60` reader - Input value of SGPIO_60"]
pub type InputValueOfSgpio60R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO61` reader - Input value of SGPIO_61"]
pub type InputValueOfSgpio61R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO62` reader - Input value of SGPIO_62"]
pub type InputValueOfSgpio62R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO63` reader - Input value of SGPIO_63"]
pub type InputValueOfSgpio63R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Input value of SGPIO_32"]
    #[inline(always)]
    pub fn input_value_of_sgpio32(&self) -> InputValueOfSgpio32R {
        InputValueOfSgpio32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input value of SGPIO_33"]
    #[inline(always)]
    pub fn input_value_of_sgpio33(&self) -> InputValueOfSgpio33R {
        InputValueOfSgpio33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input value of SGPIO_34"]
    #[inline(always)]
    pub fn input_value_of_sgpio34(&self) -> InputValueOfSgpio34R {
        InputValueOfSgpio34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input value of SGPIO_35"]
    #[inline(always)]
    pub fn input_value_of_sgpio35(&self) -> InputValueOfSgpio35R {
        InputValueOfSgpio35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input value of SGPIO_36"]
    #[inline(always)]
    pub fn input_value_of_sgpio36(&self) -> InputValueOfSgpio36R {
        InputValueOfSgpio36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input value of SGPIO_37"]
    #[inline(always)]
    pub fn input_value_of_sgpio37(&self) -> InputValueOfSgpio37R {
        InputValueOfSgpio37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input value of SGPIO_38"]
    #[inline(always)]
    pub fn input_value_of_sgpio38(&self) -> InputValueOfSgpio38R {
        InputValueOfSgpio38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input value of SGPIO_39"]
    #[inline(always)]
    pub fn input_value_of_sgpio39(&self) -> InputValueOfSgpio39R {
        InputValueOfSgpio39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Input value of SGPIO_40"]
    #[inline(always)]
    pub fn input_value_of_sgpio40(&self) -> InputValueOfSgpio40R {
        InputValueOfSgpio40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input value of SGPIO_41"]
    #[inline(always)]
    pub fn input_value_of_sgpio41(&self) -> InputValueOfSgpio41R {
        InputValueOfSgpio41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Input value of SGPIO_42"]
    #[inline(always)]
    pub fn input_value_of_sgpio42(&self) -> InputValueOfSgpio42R {
        InputValueOfSgpio42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Input value of SGPIO_43"]
    #[inline(always)]
    pub fn input_value_of_sgpio43(&self) -> InputValueOfSgpio43R {
        InputValueOfSgpio43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Input value of SGPIO_44"]
    #[inline(always)]
    pub fn input_value_of_sgpio44(&self) -> InputValueOfSgpio44R {
        InputValueOfSgpio44R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input value of SGPIO_45"]
    #[inline(always)]
    pub fn input_value_of_sgpio45(&self) -> InputValueOfSgpio45R {
        InputValueOfSgpio45R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Input value of SGPIO_46"]
    #[inline(always)]
    pub fn input_value_of_sgpio46(&self) -> InputValueOfSgpio46R {
        InputValueOfSgpio46R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Input value of SGPIO_47"]
    #[inline(always)]
    pub fn input_value_of_sgpio47(&self) -> InputValueOfSgpio47R {
        InputValueOfSgpio47R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Input value of SGPIO_48"]
    #[inline(always)]
    pub fn input_value_of_sgpio48(&self) -> InputValueOfSgpio48R {
        InputValueOfSgpio48R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Input value of SGPIO_49"]
    #[inline(always)]
    pub fn input_value_of_sgpio49(&self) -> InputValueOfSgpio49R {
        InputValueOfSgpio49R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input value of SGPIO_50"]
    #[inline(always)]
    pub fn input_value_of_sgpio50(&self) -> InputValueOfSgpio50R {
        InputValueOfSgpio50R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Input value of SGPIO_51"]
    #[inline(always)]
    pub fn input_value_of_sgpio51(&self) -> InputValueOfSgpio51R {
        InputValueOfSgpio51R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Input value of SGPIO_52"]
    #[inline(always)]
    pub fn input_value_of_sgpio52(&self) -> InputValueOfSgpio52R {
        InputValueOfSgpio52R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Input value of SGPIO_53"]
    #[inline(always)]
    pub fn input_value_of_sgpio53(&self) -> InputValueOfSgpio53R {
        InputValueOfSgpio53R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Input value of SGPIO_54"]
    #[inline(always)]
    pub fn input_value_of_sgpio54(&self) -> InputValueOfSgpio54R {
        InputValueOfSgpio54R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Input value of SGPIO_55"]
    #[inline(always)]
    pub fn input_value_of_sgpio55(&self) -> InputValueOfSgpio55R {
        InputValueOfSgpio55R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Input value of SGPIO_56"]
    #[inline(always)]
    pub fn input_value_of_sgpio56(&self) -> InputValueOfSgpio56R {
        InputValueOfSgpio56R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Input value of SGPIO_57"]
    #[inline(always)]
    pub fn input_value_of_sgpio57(&self) -> InputValueOfSgpio57R {
        InputValueOfSgpio57R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Input value of SGPIO_58"]
    #[inline(always)]
    pub fn input_value_of_sgpio58(&self) -> InputValueOfSgpio58R {
        InputValueOfSgpio58R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Input value of SGPIO_59"]
    #[inline(always)]
    pub fn input_value_of_sgpio59(&self) -> InputValueOfSgpio59R {
        InputValueOfSgpio59R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Input value of SGPIO_60"]
    #[inline(always)]
    pub fn input_value_of_sgpio60(&self) -> InputValueOfSgpio60R {
        InputValueOfSgpio60R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Input value of SGPIO_61"]
    #[inline(always)]
    pub fn input_value_of_sgpio61(&self) -> InputValueOfSgpio61R {
        InputValueOfSgpio61R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Input value of SGPIO_62"]
    #[inline(always)]
    pub fn input_value_of_sgpio62(&self) -> InputValueOfSgpio62R {
        InputValueOfSgpio62R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Input value of SGPIO_63"]
    #[inline(always)]
    pub fn input_value_of_sgpio63(&self) -> InputValueOfSgpio63R {
        InputValueOfSgpio63R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Debug Serial In Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio008Spec;
impl crate::RegisterSpec for Sgpio008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio008::R`](R) reader structure"]
impl crate::Readable for Sgpio008Spec {}
#[doc = "`write(|w| ..)` method takes [`sgpio008::W`](W) writer structure"]
impl crate::Writable for Sgpio008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO008 to value 0"]
impl crate::Resettable for Sgpio008Spec {}
