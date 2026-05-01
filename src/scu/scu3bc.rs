#[doc = "Register `SCU3BC` reader"]
pub type R = crate::R<Scu3bcSpec>;
#[doc = "Register `SCU3BC` writer"]
pub type W = crate::W<Scu3bcSpec>;
#[doc = "Field `SCUUSBSEC20` reader - SCU_USB_SEC2_0"]
pub type Scuusbsec20R = crate::BitReader;
#[doc = "Field `SCUUSBSEC20` writer - SCU_USB_SEC2_0"]
pub type Scuusbsec20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUUSBSEC22` reader - SCU_USB_SEC2_2"]
pub type Scuusbsec22R = crate::BitReader;
#[doc = "Field `SCUUSBSEC22` writer - SCU_USB_SEC2_2"]
pub type Scuusbsec22W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_USB_SEC2_0"]
    #[inline(always)]
    pub fn scuusbsec20(&self) -> Scuusbsec20R {
        Scuusbsec20R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_USB_SEC2_2"]
    #[inline(always)]
    pub fn scuusbsec22(&self) -> Scuusbsec22R {
        Scuusbsec22R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_USB_SEC2_0"]
    #[inline(always)]
    pub fn scuusbsec20(&mut self) -> Scuusbsec20W<Scu3bcSpec> {
        Scuusbsec20W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_USB_SEC2_2"]
    #[inline(always)]
    pub fn scuusbsec22(&mut self) -> Scuusbsec22W<Scu3bcSpec> {
        Scuusbsec22W::new(self, 2)
    }
}
#[doc = "USB Controler Secure Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu3bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu3bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu3bcSpec;
impl crate::RegisterSpec for Scu3bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu3bc::R`](R) reader structure"]
impl crate::Readable for Scu3bcSpec {}
#[doc = "`write(|w| ..)` method takes [`scu3bc::W`](W) writer structure"]
impl crate::Writable for Scu3bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU3BC to value 0"]
impl crate::Resettable for Scu3bcSpec {}
