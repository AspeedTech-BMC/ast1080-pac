#[doc = "Register `GPIO100` reader"]
pub type R = crate::R<Gpio100Spec>;
#[doc = "Register `GPIO100` writer"]
pub type W = crate::W<Gpio100Spec>;
#[doc = "Field `INTStatusOfGPIO000` reader - Interrupt Status of GPIO000"]
pub type IntstatusOfGpio000R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO001` reader - Interrupt Status of GPIO001"]
pub type IntstatusOfGpio001R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO002` reader - Interrupt Status of GPIO002"]
pub type IntstatusOfGpio002R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO003` reader - Interrupt Status of GPIO003"]
pub type IntstatusOfGpio003R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO004` reader - Interrupt Status of GPIO004"]
pub type IntstatusOfGpio004R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO005` reader - Interrupt Status of GPIO005"]
pub type IntstatusOfGpio005R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO006` reader - Interrupt Status of GPIO006"]
pub type IntstatusOfGpio006R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO007` reader - Interrupt Status of GPIO007"]
pub type IntstatusOfGpio007R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO008` reader - Interrupt Status of GPIO008"]
pub type IntstatusOfGpio008R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO009` reader - Interrupt Status of GPIO009"]
pub type IntstatusOfGpio009R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO010` reader - Interrupt Status of GPIO010"]
pub type IntstatusOfGpio010R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO011` reader - Interrupt Status of GPIO011"]
pub type IntstatusOfGpio011R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO012` reader - Interrupt Status of GPIO012"]
pub type IntstatusOfGpio012R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO013` reader - Interrupt Status of GPIO013"]
pub type IntstatusOfGpio013R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO014` reader - Interrupt Status of GPIO014"]
pub type IntstatusOfGpio014R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO015` reader - Interrupt Status of GPIO015"]
pub type IntstatusOfGpio015R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO016` reader - Interrupt Status of GPIO016"]
pub type IntstatusOfGpio016R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO017` reader - Interrupt Status of GPIO017"]
pub type IntstatusOfGpio017R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO018` reader - Interrupt Status of GPIO018"]
pub type IntstatusOfGpio018R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO019` reader - Interrupt Status of GPIO019"]
pub type IntstatusOfGpio019R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO020` reader - Interrupt Status of GPIO020"]
pub type IntstatusOfGpio020R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO021` reader - Interrupt Status of GPIO021"]
pub type IntstatusOfGpio021R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO022` reader - Interrupt Status of GPIO022"]
pub type IntstatusOfGpio022R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO023` reader - Interrupt Status of GPIO023"]
pub type IntstatusOfGpio023R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO024` reader - Interrupt Status of GPIO024"]
pub type IntstatusOfGpio024R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO025` reader - Interrupt Status of GPIO025"]
pub type IntstatusOfGpio025R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO026` reader - Interrupt Status of GPIO026"]
pub type IntstatusOfGpio026R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO027` reader - Interrupt Status of GPIO027"]
pub type IntstatusOfGpio027R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO028` reader - Interrupt Status of GPIO028"]
pub type IntstatusOfGpio028R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO029` reader - Interrupt Status of GPIO029"]
pub type IntstatusOfGpio029R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO030` reader - Interrupt Status of GPIO030"]
pub type IntstatusOfGpio030R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO031` reader - Interrupt Status of GPIO031"]
pub type IntstatusOfGpio031R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status of GPIO000"]
    #[inline(always)]
    pub fn intstatus_of_gpio000(&self) -> IntstatusOfGpio000R {
        IntstatusOfGpio000R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status of GPIO001"]
    #[inline(always)]
    pub fn intstatus_of_gpio001(&self) -> IntstatusOfGpio001R {
        IntstatusOfGpio001R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status of GPIO002"]
    #[inline(always)]
    pub fn intstatus_of_gpio002(&self) -> IntstatusOfGpio002R {
        IntstatusOfGpio002R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status of GPIO003"]
    #[inline(always)]
    pub fn intstatus_of_gpio003(&self) -> IntstatusOfGpio003R {
        IntstatusOfGpio003R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status of GPIO004"]
    #[inline(always)]
    pub fn intstatus_of_gpio004(&self) -> IntstatusOfGpio004R {
        IntstatusOfGpio004R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status of GPIO005"]
    #[inline(always)]
    pub fn intstatus_of_gpio005(&self) -> IntstatusOfGpio005R {
        IntstatusOfGpio005R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status of GPIO006"]
    #[inline(always)]
    pub fn intstatus_of_gpio006(&self) -> IntstatusOfGpio006R {
        IntstatusOfGpio006R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status of GPIO007"]
    #[inline(always)]
    pub fn intstatus_of_gpio007(&self) -> IntstatusOfGpio007R {
        IntstatusOfGpio007R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Status of GPIO008"]
    #[inline(always)]
    pub fn intstatus_of_gpio008(&self) -> IntstatusOfGpio008R {
        IntstatusOfGpio008R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Status of GPIO009"]
    #[inline(always)]
    pub fn intstatus_of_gpio009(&self) -> IntstatusOfGpio009R {
        IntstatusOfGpio009R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Status of GPIO010"]
    #[inline(always)]
    pub fn intstatus_of_gpio010(&self) -> IntstatusOfGpio010R {
        IntstatusOfGpio010R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Status of GPIO011"]
    #[inline(always)]
    pub fn intstatus_of_gpio011(&self) -> IntstatusOfGpio011R {
        IntstatusOfGpio011R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Status of GPIO012"]
    #[inline(always)]
    pub fn intstatus_of_gpio012(&self) -> IntstatusOfGpio012R {
        IntstatusOfGpio012R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Status of GPIO013"]
    #[inline(always)]
    pub fn intstatus_of_gpio013(&self) -> IntstatusOfGpio013R {
        IntstatusOfGpio013R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Status of GPIO014"]
    #[inline(always)]
    pub fn intstatus_of_gpio014(&self) -> IntstatusOfGpio014R {
        IntstatusOfGpio014R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Status of GPIO015"]
    #[inline(always)]
    pub fn intstatus_of_gpio015(&self) -> IntstatusOfGpio015R {
        IntstatusOfGpio015R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Status of GPIO016"]
    #[inline(always)]
    pub fn intstatus_of_gpio016(&self) -> IntstatusOfGpio016R {
        IntstatusOfGpio016R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Status of GPIO017"]
    #[inline(always)]
    pub fn intstatus_of_gpio017(&self) -> IntstatusOfGpio017R {
        IntstatusOfGpio017R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Status of GPIO018"]
    #[inline(always)]
    pub fn intstatus_of_gpio018(&self) -> IntstatusOfGpio018R {
        IntstatusOfGpio018R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Status of GPIO019"]
    #[inline(always)]
    pub fn intstatus_of_gpio019(&self) -> IntstatusOfGpio019R {
        IntstatusOfGpio019R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Status of GPIO020"]
    #[inline(always)]
    pub fn intstatus_of_gpio020(&self) -> IntstatusOfGpio020R {
        IntstatusOfGpio020R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Status of GPIO021"]
    #[inline(always)]
    pub fn intstatus_of_gpio021(&self) -> IntstatusOfGpio021R {
        IntstatusOfGpio021R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Status of GPIO022"]
    #[inline(always)]
    pub fn intstatus_of_gpio022(&self) -> IntstatusOfGpio022R {
        IntstatusOfGpio022R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Status of GPIO023"]
    #[inline(always)]
    pub fn intstatus_of_gpio023(&self) -> IntstatusOfGpio023R {
        IntstatusOfGpio023R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Status of GPIO024"]
    #[inline(always)]
    pub fn intstatus_of_gpio024(&self) -> IntstatusOfGpio024R {
        IntstatusOfGpio024R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt Status of GPIO025"]
    #[inline(always)]
    pub fn intstatus_of_gpio025(&self) -> IntstatusOfGpio025R {
        IntstatusOfGpio025R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt Status of GPIO026"]
    #[inline(always)]
    pub fn intstatus_of_gpio026(&self) -> IntstatusOfGpio026R {
        IntstatusOfGpio026R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt Status of GPIO027"]
    #[inline(always)]
    pub fn intstatus_of_gpio027(&self) -> IntstatusOfGpio027R {
        IntstatusOfGpio027R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt Status of GPIO028"]
    #[inline(always)]
    pub fn intstatus_of_gpio028(&self) -> IntstatusOfGpio028R {
        IntstatusOfGpio028R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt Status of GPIO029"]
    #[inline(always)]
    pub fn intstatus_of_gpio029(&self) -> IntstatusOfGpio029R {
        IntstatusOfGpio029R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt Status of GPIO030"]
    #[inline(always)]
    pub fn intstatus_of_gpio030(&self) -> IntstatusOfGpio030R {
        IntstatusOfGpio030R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Status of GPIO031"]
    #[inline(always)]
    pub fn intstatus_of_gpio031(&self) -> IntstatusOfGpio031R {
        IntstatusOfGpio031R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt Status Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio100::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio100::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio100Spec;
impl crate::RegisterSpec for Gpio100Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio100::R`](R) reader structure"]
impl crate::Readable for Gpio100Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio100::W`](W) writer structure"]
impl crate::Writable for Gpio100Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO100 to value 0"]
impl crate::Resettable for Gpio100Spec {}
