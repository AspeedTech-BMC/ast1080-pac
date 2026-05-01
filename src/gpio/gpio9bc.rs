#[doc = "Register `GPIO9BC` reader"]
pub type R = crate::R<Gpio9bcSpec>;
#[doc = "Register `GPIO9BC` writer"]
pub type W = crate::W<Gpio9bcSpec>;
#[doc = "Field `GPIO172ReadPrivilegeOfMaster` reader - GPIO172 Read Privilege of Master"]
pub type Gpio172readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO172ReadPrivilegeOfMaster` writer - GPIO172 Read Privilege of Master"]
pub type Gpio172readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO173ReadPrivilegeOfMaster` reader - GPIO173 Read Privilege of Master"]
pub type Gpio173readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO173ReadPrivilegeOfMaster` writer - GPIO173 Read Privilege of Master"]
pub type Gpio173readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO174ReadPrivilegeOfMaster` reader - GPIO174 Read Privilege of Master"]
pub type Gpio174readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO174ReadPrivilegeOfMaster` writer - GPIO174 Read Privilege of Master"]
pub type Gpio174readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO175ReadPrivilegeOfMaster` reader - GPIO175 Read Privilege of Master"]
pub type Gpio175readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO175ReadPrivilegeOfMaster` writer - GPIO175 Read Privilege of Master"]
pub type Gpio175readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO172 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio172read_privilege_of_master(&self) -> Gpio172readPrivilegeOfMasterR {
        Gpio172readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO173 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio173read_privilege_of_master(&self) -> Gpio173readPrivilegeOfMasterR {
        Gpio173readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO174 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio174read_privilege_of_master(&self) -> Gpio174readPrivilegeOfMasterR {
        Gpio174readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO175 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio175read_privilege_of_master(&self) -> Gpio175readPrivilegeOfMasterR {
        Gpio175readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO172 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio172read_privilege_of_master(
        &mut self,
    ) -> Gpio172readPrivilegeOfMasterW<Gpio9bcSpec> {
        Gpio172readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO173 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio173read_privilege_of_master(
        &mut self,
    ) -> Gpio173readPrivilegeOfMasterW<Gpio9bcSpec> {
        Gpio173readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO174 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio174read_privilege_of_master(
        &mut self,
    ) -> Gpio174readPrivilegeOfMasterW<Gpio9bcSpec> {
        Gpio174readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO175 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio175read_privilege_of_master(
        &mut self,
    ) -> Gpio175readPrivilegeOfMasterW<Gpio9bcSpec> {
        Gpio175readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#43\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio9bcSpec;
impl crate::RegisterSpec for Gpio9bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio9bc::R`](R) reader structure"]
impl crate::Readable for Gpio9bcSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio9bc::W`](W) writer structure"]
impl crate::Writable for Gpio9bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO9BC to value 0xffff_ffff"]
impl crate::Resettable for Gpio9bcSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
