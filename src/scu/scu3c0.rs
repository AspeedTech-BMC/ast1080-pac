#[doc = "Register `SCU3C0` reader"]
pub type R = crate::R<Scu3c0Spec>;
#[doc = "Register `SCU3C0` writer"]
pub type W = crate::W<Scu3c0Spec>;
#[doc = "Field `SCUUSBSEC30` reader - SCU_USB_SEC3_0"]
pub type Scuusbsec30R = crate::BitReader;
#[doc = "Field `SCUUSBSEC30` writer - SCU_USB_SEC3_0"]
pub type Scuusbsec30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUUSBSEC32` reader - SCU_USB_SEC3_2"]
pub type Scuusbsec32R = crate::BitReader;
#[doc = "Field `SCUUSBSEC32` writer - SCU_USB_SEC3_2"]
pub type Scuusbsec32W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_USB_SEC3_0"]
    #[inline(always)]
    pub fn scuusbsec30(&self) -> Scuusbsec30R {
        Scuusbsec30R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_USB_SEC3_2"]
    #[inline(always)]
    pub fn scuusbsec32(&self) -> Scuusbsec32R {
        Scuusbsec32R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_USB_SEC3_0"]
    #[inline(always)]
    pub fn scuusbsec30(&mut self) -> Scuusbsec30W<Scu3c0Spec> {
        Scuusbsec30W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_USB_SEC3_2"]
    #[inline(always)]
    pub fn scuusbsec32(&mut self) -> Scuusbsec32W<Scu3c0Spec> {
        Scuusbsec32W::new(self, 2)
    }
}
#[doc = "USB Controler Secure Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu3c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu3c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu3c0Spec;
impl crate::RegisterSpec for Scu3c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu3c0::R`](R) reader structure"]
impl crate::Readable for Scu3c0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu3c0::W`](W) writer structure"]
impl crate::Writable for Scu3c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU3C0 to value 0"]
impl crate::Resettable for Scu3c0Spec {}
