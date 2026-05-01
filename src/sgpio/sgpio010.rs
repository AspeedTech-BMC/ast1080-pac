#[doc = "Register `SGPIO010` reader"]
pub type R = crate::R<Sgpio010Spec>;
#[doc = "Register `SGPIO010` writer"]
pub type W = crate::W<Sgpio010Spec>;
#[doc = "Field `InputValueOfSGPIO96` reader - Input value of SGPIO_96"]
pub type InputValueOfSgpio96R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO97` reader - Input value of SGPIO_97"]
pub type InputValueOfSgpio97R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO98` reader - Input value of SGPIO_98"]
pub type InputValueOfSgpio98R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO99` reader - Input value of SGPIO_99"]
pub type InputValueOfSgpio99R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO100` reader - Input value of SGPIO_100"]
pub type InputValueOfSgpio100R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO101` reader - Input value of SGPIO_101"]
pub type InputValueOfSgpio101R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO102` reader - Input value of SGPIO_102"]
pub type InputValueOfSgpio102R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO103` reader - Input value of SGPIO_103"]
pub type InputValueOfSgpio103R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO104` reader - Input value of SGPIO_104"]
pub type InputValueOfSgpio104R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO105` reader - Input value of SGPIO_105"]
pub type InputValueOfSgpio105R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO106` reader - Input value of SGPIO_106"]
pub type InputValueOfSgpio106R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO107` reader - Input value of SGPIO_107"]
pub type InputValueOfSgpio107R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO108` reader - Input value of SGPIO_108"]
pub type InputValueOfSgpio108R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO109` reader - Input value of SGPIO_109"]
pub type InputValueOfSgpio109R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO110` reader - Input value of SGPIO_110"]
pub type InputValueOfSgpio110R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO111` reader - Input value of SGPIO_111"]
pub type InputValueOfSgpio111R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO112` reader - Input value of SGPIO_112"]
pub type InputValueOfSgpio112R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO113` reader - Input value of SGPIO_113"]
pub type InputValueOfSgpio113R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO114` reader - Input value of SGPIO_114"]
pub type InputValueOfSgpio114R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO115` reader - Input value of SGPIO_115"]
pub type InputValueOfSgpio115R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO116` reader - Input value of SGPIO_116"]
pub type InputValueOfSgpio116R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO117` reader - Input value of SGPIO_117"]
pub type InputValueOfSgpio117R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO118` reader - Input value of SGPIO_118"]
pub type InputValueOfSgpio118R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO119` reader - Input value of SGPIO_119"]
pub type InputValueOfSgpio119R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO120` reader - Input value of SGPIO_120"]
pub type InputValueOfSgpio120R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO121` reader - Input value of SGPIO_121"]
pub type InputValueOfSgpio121R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO122` reader - Input value of SGPIO_122"]
pub type InputValueOfSgpio122R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO123` reader - Input value of SGPIO_123"]
pub type InputValueOfSgpio123R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO124` reader - Input value of SGPIO_124"]
pub type InputValueOfSgpio124R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO125` reader - Input value of SGPIO_125"]
pub type InputValueOfSgpio125R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO126` reader - Input value of SGPIO_126"]
pub type InputValueOfSgpio126R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO127` reader - Input value of SGPIO_127"]
pub type InputValueOfSgpio127R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Input value of SGPIO_96"]
    #[inline(always)]
    pub fn input_value_of_sgpio96(&self) -> InputValueOfSgpio96R {
        InputValueOfSgpio96R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input value of SGPIO_97"]
    #[inline(always)]
    pub fn input_value_of_sgpio97(&self) -> InputValueOfSgpio97R {
        InputValueOfSgpio97R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input value of SGPIO_98"]
    #[inline(always)]
    pub fn input_value_of_sgpio98(&self) -> InputValueOfSgpio98R {
        InputValueOfSgpio98R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input value of SGPIO_99"]
    #[inline(always)]
    pub fn input_value_of_sgpio99(&self) -> InputValueOfSgpio99R {
        InputValueOfSgpio99R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input value of SGPIO_100"]
    #[inline(always)]
    pub fn input_value_of_sgpio100(&self) -> InputValueOfSgpio100R {
        InputValueOfSgpio100R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input value of SGPIO_101"]
    #[inline(always)]
    pub fn input_value_of_sgpio101(&self) -> InputValueOfSgpio101R {
        InputValueOfSgpio101R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input value of SGPIO_102"]
    #[inline(always)]
    pub fn input_value_of_sgpio102(&self) -> InputValueOfSgpio102R {
        InputValueOfSgpio102R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input value of SGPIO_103"]
    #[inline(always)]
    pub fn input_value_of_sgpio103(&self) -> InputValueOfSgpio103R {
        InputValueOfSgpio103R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Input value of SGPIO_104"]
    #[inline(always)]
    pub fn input_value_of_sgpio104(&self) -> InputValueOfSgpio104R {
        InputValueOfSgpio104R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input value of SGPIO_105"]
    #[inline(always)]
    pub fn input_value_of_sgpio105(&self) -> InputValueOfSgpio105R {
        InputValueOfSgpio105R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Input value of SGPIO_106"]
    #[inline(always)]
    pub fn input_value_of_sgpio106(&self) -> InputValueOfSgpio106R {
        InputValueOfSgpio106R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Input value of SGPIO_107"]
    #[inline(always)]
    pub fn input_value_of_sgpio107(&self) -> InputValueOfSgpio107R {
        InputValueOfSgpio107R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Input value of SGPIO_108"]
    #[inline(always)]
    pub fn input_value_of_sgpio108(&self) -> InputValueOfSgpio108R {
        InputValueOfSgpio108R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input value of SGPIO_109"]
    #[inline(always)]
    pub fn input_value_of_sgpio109(&self) -> InputValueOfSgpio109R {
        InputValueOfSgpio109R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Input value of SGPIO_110"]
    #[inline(always)]
    pub fn input_value_of_sgpio110(&self) -> InputValueOfSgpio110R {
        InputValueOfSgpio110R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Input value of SGPIO_111"]
    #[inline(always)]
    pub fn input_value_of_sgpio111(&self) -> InputValueOfSgpio111R {
        InputValueOfSgpio111R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Input value of SGPIO_112"]
    #[inline(always)]
    pub fn input_value_of_sgpio112(&self) -> InputValueOfSgpio112R {
        InputValueOfSgpio112R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Input value of SGPIO_113"]
    #[inline(always)]
    pub fn input_value_of_sgpio113(&self) -> InputValueOfSgpio113R {
        InputValueOfSgpio113R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input value of SGPIO_114"]
    #[inline(always)]
    pub fn input_value_of_sgpio114(&self) -> InputValueOfSgpio114R {
        InputValueOfSgpio114R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Input value of SGPIO_115"]
    #[inline(always)]
    pub fn input_value_of_sgpio115(&self) -> InputValueOfSgpio115R {
        InputValueOfSgpio115R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Input value of SGPIO_116"]
    #[inline(always)]
    pub fn input_value_of_sgpio116(&self) -> InputValueOfSgpio116R {
        InputValueOfSgpio116R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Input value of SGPIO_117"]
    #[inline(always)]
    pub fn input_value_of_sgpio117(&self) -> InputValueOfSgpio117R {
        InputValueOfSgpio117R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Input value of SGPIO_118"]
    #[inline(always)]
    pub fn input_value_of_sgpio118(&self) -> InputValueOfSgpio118R {
        InputValueOfSgpio118R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Input value of SGPIO_119"]
    #[inline(always)]
    pub fn input_value_of_sgpio119(&self) -> InputValueOfSgpio119R {
        InputValueOfSgpio119R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Input value of SGPIO_120"]
    #[inline(always)]
    pub fn input_value_of_sgpio120(&self) -> InputValueOfSgpio120R {
        InputValueOfSgpio120R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Input value of SGPIO_121"]
    #[inline(always)]
    pub fn input_value_of_sgpio121(&self) -> InputValueOfSgpio121R {
        InputValueOfSgpio121R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Input value of SGPIO_122"]
    #[inline(always)]
    pub fn input_value_of_sgpio122(&self) -> InputValueOfSgpio122R {
        InputValueOfSgpio122R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Input value of SGPIO_123"]
    #[inline(always)]
    pub fn input_value_of_sgpio123(&self) -> InputValueOfSgpio123R {
        InputValueOfSgpio123R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Input value of SGPIO_124"]
    #[inline(always)]
    pub fn input_value_of_sgpio124(&self) -> InputValueOfSgpio124R {
        InputValueOfSgpio124R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Input value of SGPIO_125"]
    #[inline(always)]
    pub fn input_value_of_sgpio125(&self) -> InputValueOfSgpio125R {
        InputValueOfSgpio125R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Input value of SGPIO_126"]
    #[inline(always)]
    pub fn input_value_of_sgpio126(&self) -> InputValueOfSgpio126R {
        InputValueOfSgpio126R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Input value of SGPIO_127"]
    #[inline(always)]
    pub fn input_value_of_sgpio127(&self) -> InputValueOfSgpio127R {
        InputValueOfSgpio127R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Debug Serial In Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio010Spec;
impl crate::RegisterSpec for Sgpio010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio010::R`](R) reader structure"]
impl crate::Readable for Sgpio010Spec {}
#[doc = "`write(|w| ..)` method takes [`sgpio010::W`](W) writer structure"]
impl crate::Writable for Sgpio010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO010 to value 0"]
impl crate::Resettable for Sgpio010Spec {}
