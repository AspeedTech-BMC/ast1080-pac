#[doc = "Register `GPIO838` reader"]
pub type R = crate::R<Gpio838Spec>;
#[doc = "Register `GPIO838` writer"]
pub type W = crate::W<Gpio838Spec>;
#[doc = "Field `GPIO040WrPrivilegeOfMaster` reader - GPIO040 Write Privilege of Master"]
pub type Gpio040wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO040WrPrivilegeOfMaster` writer - GPIO040 Write Privilege of Master"]
pub type Gpio040wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO041WrPrivilegeOfMaster` reader - GPIO041 Write Privilege of Master"]
pub type Gpio041wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO041WrPrivilegeOfMaster` writer - GPIO041 Write Privilege of Master"]
pub type Gpio041wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO042WrPrivilegeOfMaster` reader - GPIO042 Write Privilege of Master"]
pub type Gpio042wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO042WrPrivilegeOfMaster` writer - GPIO042 Write Privilege of Master"]
pub type Gpio042wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO043WrPrivilegeOfMaster` reader - GPIO043 Write Privilege of Master"]
pub type Gpio043wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO043WrPrivilegeOfMaster` writer - GPIO043 Write Privilege of Master"]
pub type Gpio043wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO040 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio040wr_privilege_of_master(&self) -> Gpio040wrPrivilegeOfMasterR {
        Gpio040wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO041 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio041wr_privilege_of_master(&self) -> Gpio041wrPrivilegeOfMasterR {
        Gpio041wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO042 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio042wr_privilege_of_master(&self) -> Gpio042wrPrivilegeOfMasterR {
        Gpio042wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO043 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio043wr_privilege_of_master(&self) -> Gpio043wrPrivilegeOfMasterR {
        Gpio043wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO040 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio040wr_privilege_of_master(&mut self) -> Gpio040wrPrivilegeOfMasterW<Gpio838Spec> {
        Gpio040wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO041 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio041wr_privilege_of_master(&mut self) -> Gpio041wrPrivilegeOfMasterW<Gpio838Spec> {
        Gpio041wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO042 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio042wr_privilege_of_master(&mut self) -> Gpio042wrPrivilegeOfMasterW<Gpio838Spec> {
        Gpio042wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO043 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio043wr_privilege_of_master(&mut self) -> Gpio043wrPrivilegeOfMasterW<Gpio838Spec> {
        Gpio043wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#10\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio838::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio838::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio838Spec;
impl crate::RegisterSpec for Gpio838Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio838::R`](R) reader structure"]
impl crate::Readable for Gpio838Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio838::W`](W) writer structure"]
impl crate::Writable for Gpio838Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO838 to value 0xffff_ffff"]
impl crate::Resettable for Gpio838Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
