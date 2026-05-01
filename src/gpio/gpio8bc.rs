#[doc = "Register `GPIO8BC` reader"]
pub type R = crate::R<Gpio8bcSpec>;
#[doc = "Register `GPIO8BC` writer"]
pub type W = crate::W<Gpio8bcSpec>;
#[doc = "Field `GPIO172WrPrivilegeOfMaster` reader - GPIO172 Write Privilege of Master"]
pub type Gpio172wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO172WrPrivilegeOfMaster` writer - GPIO172 Write Privilege of Master"]
pub type Gpio172wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO173WrPrivilegeOfMaster` reader - GPIO173 Write Privilege of Master"]
pub type Gpio173wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO173WrPrivilegeOfMaster` writer - GPIO173 Write Privilege of Master"]
pub type Gpio173wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO174WrPrivilegeOfMaster` reader - GPIO174 Write Privilege of Master"]
pub type Gpio174wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO174WrPrivilegeOfMaster` writer - GPIO174 Write Privilege of Master"]
pub type Gpio174wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO175WrPrivilegeOfMaster` reader - GPIO175 Write Privilege of Master"]
pub type Gpio175wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO175WrPrivilegeOfMaster` writer - GPIO175 Write Privilege of Master"]
pub type Gpio175wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO172 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio172wr_privilege_of_master(&self) -> Gpio172wrPrivilegeOfMasterR {
        Gpio172wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO173 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio173wr_privilege_of_master(&self) -> Gpio173wrPrivilegeOfMasterR {
        Gpio173wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO174 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio174wr_privilege_of_master(&self) -> Gpio174wrPrivilegeOfMasterR {
        Gpio174wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO175 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio175wr_privilege_of_master(&self) -> Gpio175wrPrivilegeOfMasterR {
        Gpio175wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO172 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio172wr_privilege_of_master(&mut self) -> Gpio172wrPrivilegeOfMasterW<Gpio8bcSpec> {
        Gpio172wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO173 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio173wr_privilege_of_master(&mut self) -> Gpio173wrPrivilegeOfMasterW<Gpio8bcSpec> {
        Gpio173wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO174 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio174wr_privilege_of_master(&mut self) -> Gpio174wrPrivilegeOfMasterW<Gpio8bcSpec> {
        Gpio174wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO175 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio175wr_privilege_of_master(&mut self) -> Gpio175wrPrivilegeOfMasterW<Gpio8bcSpec> {
        Gpio175wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#43\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio8bcSpec;
impl crate::RegisterSpec for Gpio8bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio8bc::R`](R) reader structure"]
impl crate::Readable for Gpio8bcSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio8bc::W`](W) writer structure"]
impl crate::Writable for Gpio8bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO8BC to value 0xffff_ffff"]
impl crate::Resettable for Gpio8bcSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
