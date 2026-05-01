#[doc = "Register `SGPIO28` reader"]
pub type R = crate::R<Sgpio28Spec>;
#[doc = "Register `SGPIO28` writer"]
pub type W = crate::W<Sgpio28Spec>;
#[doc = "Field `OutputValueOfSGPIO32` reader - Output value of SGPIO_32"]
pub type OutputValueOfSgpio32R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO33` reader - Output value of SGPIO_33"]
pub type OutputValueOfSgpio33R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO34` reader - Output value of SGPIO_34"]
pub type OutputValueOfSgpio34R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO35` reader - Output value of SGPIO_35"]
pub type OutputValueOfSgpio35R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO36` reader - Output value of SGPIO_36"]
pub type OutputValueOfSgpio36R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO37` reader - Output value of SGPIO_37"]
pub type OutputValueOfSgpio37R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO38` reader - Output value of SGPIO_38"]
pub type OutputValueOfSgpio38R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO39` reader - Output value of SGPIO_39"]
pub type OutputValueOfSgpio39R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO40` reader - Output value of SGPIO_40"]
pub type OutputValueOfSgpio40R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO41` reader - Output value of SGPIO_41"]
pub type OutputValueOfSgpio41R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO42` reader - Output value of SGPIO_42"]
pub type OutputValueOfSgpio42R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO43` reader - Output value of SGPIO_43"]
pub type OutputValueOfSgpio43R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO44` reader - Output value of SGPIO_44"]
pub type OutputValueOfSgpio44R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO45` reader - Output value of SGPIO_45"]
pub type OutputValueOfSgpio45R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO46` reader - Output value of SGPIO_46"]
pub type OutputValueOfSgpio46R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO47` reader - Output value of SGPIO_47"]
pub type OutputValueOfSgpio47R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO48` reader - Output value of SGPIO_48"]
pub type OutputValueOfSgpio48R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO49` reader - Output value of SGPIO_49"]
pub type OutputValueOfSgpio49R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO50` reader - Output value of SGPIO_50"]
pub type OutputValueOfSgpio50R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO51` reader - Output value of SGPIO_51"]
pub type OutputValueOfSgpio51R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO52` reader - Output value of SGPIO_52"]
pub type OutputValueOfSgpio52R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO53` reader - Output value of SGPIO_53"]
pub type OutputValueOfSgpio53R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO54` reader - Output value of SGPIO_54"]
pub type OutputValueOfSgpio54R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO55` reader - Output value of SGPIO_55"]
pub type OutputValueOfSgpio55R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO56` reader - Output value of SGPIO_56"]
pub type OutputValueOfSgpio56R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO57` reader - Output value of SGPIO_57"]
pub type OutputValueOfSgpio57R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO58` reader - Output value of SGPIO_58"]
pub type OutputValueOfSgpio58R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO59` reader - Output value of SGPIO_59"]
pub type OutputValueOfSgpio59R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO60` reader - Output value of SGPIO_60"]
pub type OutputValueOfSgpio60R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO61` reader - Output value of SGPIO_61"]
pub type OutputValueOfSgpio61R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO62` reader - Output value of SGPIO_62"]
pub type OutputValueOfSgpio62R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO63` reader - Output value of SGPIO_63"]
pub type OutputValueOfSgpio63R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Output value of SGPIO_32"]
    #[inline(always)]
    pub fn output_value_of_sgpio32(&self) -> OutputValueOfSgpio32R {
        OutputValueOfSgpio32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output value of SGPIO_33"]
    #[inline(always)]
    pub fn output_value_of_sgpio33(&self) -> OutputValueOfSgpio33R {
        OutputValueOfSgpio33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output value of SGPIO_34"]
    #[inline(always)]
    pub fn output_value_of_sgpio34(&self) -> OutputValueOfSgpio34R {
        OutputValueOfSgpio34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output value of SGPIO_35"]
    #[inline(always)]
    pub fn output_value_of_sgpio35(&self) -> OutputValueOfSgpio35R {
        OutputValueOfSgpio35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output value of SGPIO_36"]
    #[inline(always)]
    pub fn output_value_of_sgpio36(&self) -> OutputValueOfSgpio36R {
        OutputValueOfSgpio36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Output value of SGPIO_37"]
    #[inline(always)]
    pub fn output_value_of_sgpio37(&self) -> OutputValueOfSgpio37R {
        OutputValueOfSgpio37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Output value of SGPIO_38"]
    #[inline(always)]
    pub fn output_value_of_sgpio38(&self) -> OutputValueOfSgpio38R {
        OutputValueOfSgpio38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output value of SGPIO_39"]
    #[inline(always)]
    pub fn output_value_of_sgpio39(&self) -> OutputValueOfSgpio39R {
        OutputValueOfSgpio39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output value of SGPIO_40"]
    #[inline(always)]
    pub fn output_value_of_sgpio40(&self) -> OutputValueOfSgpio40R {
        OutputValueOfSgpio40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output value of SGPIO_41"]
    #[inline(always)]
    pub fn output_value_of_sgpio41(&self) -> OutputValueOfSgpio41R {
        OutputValueOfSgpio41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output value of SGPIO_42"]
    #[inline(always)]
    pub fn output_value_of_sgpio42(&self) -> OutputValueOfSgpio42R {
        OutputValueOfSgpio42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output value of SGPIO_43"]
    #[inline(always)]
    pub fn output_value_of_sgpio43(&self) -> OutputValueOfSgpio43R {
        OutputValueOfSgpio43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output value of SGPIO_44"]
    #[inline(always)]
    pub fn output_value_of_sgpio44(&self) -> OutputValueOfSgpio44R {
        OutputValueOfSgpio44R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output value of SGPIO_45"]
    #[inline(always)]
    pub fn output_value_of_sgpio45(&self) -> OutputValueOfSgpio45R {
        OutputValueOfSgpio45R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output value of SGPIO_46"]
    #[inline(always)]
    pub fn output_value_of_sgpio46(&self) -> OutputValueOfSgpio46R {
        OutputValueOfSgpio46R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Output value of SGPIO_47"]
    #[inline(always)]
    pub fn output_value_of_sgpio47(&self) -> OutputValueOfSgpio47R {
        OutputValueOfSgpio47R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output value of SGPIO_48"]
    #[inline(always)]
    pub fn output_value_of_sgpio48(&self) -> OutputValueOfSgpio48R {
        OutputValueOfSgpio48R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Output value of SGPIO_49"]
    #[inline(always)]
    pub fn output_value_of_sgpio49(&self) -> OutputValueOfSgpio49R {
        OutputValueOfSgpio49R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output value of SGPIO_50"]
    #[inline(always)]
    pub fn output_value_of_sgpio50(&self) -> OutputValueOfSgpio50R {
        OutputValueOfSgpio50R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output value of SGPIO_51"]
    #[inline(always)]
    pub fn output_value_of_sgpio51(&self) -> OutputValueOfSgpio51R {
        OutputValueOfSgpio51R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output value of SGPIO_52"]
    #[inline(always)]
    pub fn output_value_of_sgpio52(&self) -> OutputValueOfSgpio52R {
        OutputValueOfSgpio52R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Output value of SGPIO_53"]
    #[inline(always)]
    pub fn output_value_of_sgpio53(&self) -> OutputValueOfSgpio53R {
        OutputValueOfSgpio53R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Output value of SGPIO_54"]
    #[inline(always)]
    pub fn output_value_of_sgpio54(&self) -> OutputValueOfSgpio54R {
        OutputValueOfSgpio54R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Output value of SGPIO_55"]
    #[inline(always)]
    pub fn output_value_of_sgpio55(&self) -> OutputValueOfSgpio55R {
        OutputValueOfSgpio55R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Output value of SGPIO_56"]
    #[inline(always)]
    pub fn output_value_of_sgpio56(&self) -> OutputValueOfSgpio56R {
        OutputValueOfSgpio56R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Output value of SGPIO_57"]
    #[inline(always)]
    pub fn output_value_of_sgpio57(&self) -> OutputValueOfSgpio57R {
        OutputValueOfSgpio57R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Output value of SGPIO_58"]
    #[inline(always)]
    pub fn output_value_of_sgpio58(&self) -> OutputValueOfSgpio58R {
        OutputValueOfSgpio58R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Output value of SGPIO_59"]
    #[inline(always)]
    pub fn output_value_of_sgpio59(&self) -> OutputValueOfSgpio59R {
        OutputValueOfSgpio59R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Output value of SGPIO_60"]
    #[inline(always)]
    pub fn output_value_of_sgpio60(&self) -> OutputValueOfSgpio60R {
        OutputValueOfSgpio60R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Output value of SGPIO_61"]
    #[inline(always)]
    pub fn output_value_of_sgpio61(&self) -> OutputValueOfSgpio61R {
        OutputValueOfSgpio61R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Output value of SGPIO_62"]
    #[inline(always)]
    pub fn output_value_of_sgpio62(&self) -> OutputValueOfSgpio62R {
        OutputValueOfSgpio62R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Output value of SGPIO_63"]
    #[inline(always)]
    pub fn output_value_of_sgpio63(&self) -> OutputValueOfSgpio63R {
        OutputValueOfSgpio63R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Debug Serial Out Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio28Spec;
impl crate::RegisterSpec for Sgpio28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio28::R`](R) reader structure"]
impl crate::Readable for Sgpio28Spec {}
#[doc = "`write(|w| ..)` method takes [`sgpio28::W`](W) writer structure"]
impl crate::Writable for Sgpio28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO28 to value 0"]
impl crate::Resettable for Sgpio28Spec {}
