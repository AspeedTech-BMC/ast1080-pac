#[doc = "Register `GPIO8D0` reader"]
pub type R = crate::R<Gpio8d0Spec>;
#[doc = "Register `GPIO8D0` writer"]
pub type W = crate::W<Gpio8d0Spec>;
#[doc = "Field `GPIO192WrPrivilegeOfMaster` reader - GPIO192 Write Privilege of Master"]
pub type Gpio192wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO192WrPrivilegeOfMaster` writer - GPIO192 Write Privilege of Master"]
pub type Gpio192wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO193WrPrivilegeOfMaster` reader - GPIO193 Write Privilege of Master"]
pub type Gpio193wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO193WrPrivilegeOfMaster` writer - GPIO193 Write Privilege of Master"]
pub type Gpio193wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO192 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio192wr_privilege_of_master(&self) -> Gpio192wrPrivilegeOfMasterR {
        Gpio192wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO193 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio193wr_privilege_of_master(&self) -> Gpio193wrPrivilegeOfMasterR {
        Gpio193wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO192 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio192wr_privilege_of_master(&mut self) -> Gpio192wrPrivilegeOfMasterW<Gpio8d0Spec> {
        Gpio192wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO193 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio193wr_privilege_of_master(&mut self) -> Gpio193wrPrivilegeOfMasterW<Gpio8d0Spec> {
        Gpio193wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpio8d0Spec> {
        Reserved1W::new(self, 16)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#48\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio8d0Spec;
impl crate::RegisterSpec for Gpio8d0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio8d0::R`](R) reader structure"]
impl crate::Readable for Gpio8d0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio8d0::W`](W) writer structure"]
impl crate::Writable for Gpio8d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO8D0 to value 0xffff_ffff"]
impl crate::Resettable for Gpio8d0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
