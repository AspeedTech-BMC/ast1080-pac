#[doc = "Register `GPIO99C` reader"]
pub type R = crate::R<Gpio99cSpec>;
#[doc = "Register `GPIO99C` writer"]
pub type W = crate::W<Gpio99cSpec>;
#[doc = "Field `GPIO140ReadPrivilegeOfMaster` reader - GPIO140 Read Privilege of Master"]
pub type Gpio140readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO140ReadPrivilegeOfMaster` writer - GPIO140 Read Privilege of Master"]
pub type Gpio140readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO141ReadPrivilegeOfMaster` reader - GPIO141 Read Privilege of Master"]
pub type Gpio141readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO141ReadPrivilegeOfMaster` writer - GPIO141 Read Privilege of Master"]
pub type Gpio141readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO142ReadPrivilegeOfMaster` reader - GPIO142 Read Privilege of Master"]
pub type Gpio142readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO142ReadPrivilegeOfMaster` writer - GPIO142 Read Privilege of Master"]
pub type Gpio142readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO143ReadPrivilegeOfMaster` reader - GPIO143 Read Privilege of Master"]
pub type Gpio143readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO143ReadPrivilegeOfMaster` writer - GPIO143 Read Privilege of Master"]
pub type Gpio143readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO140 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio140read_privilege_of_master(&self) -> Gpio140readPrivilegeOfMasterR {
        Gpio140readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO141 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio141read_privilege_of_master(&self) -> Gpio141readPrivilegeOfMasterR {
        Gpio141readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO142 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio142read_privilege_of_master(&self) -> Gpio142readPrivilegeOfMasterR {
        Gpio142readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO143 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio143read_privilege_of_master(&self) -> Gpio143readPrivilegeOfMasterR {
        Gpio143readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO140 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio140read_privilege_of_master(
        &mut self,
    ) -> Gpio140readPrivilegeOfMasterW<Gpio99cSpec> {
        Gpio140readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO141 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio141read_privilege_of_master(
        &mut self,
    ) -> Gpio141readPrivilegeOfMasterW<Gpio99cSpec> {
        Gpio141readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO142 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio142read_privilege_of_master(
        &mut self,
    ) -> Gpio142readPrivilegeOfMasterW<Gpio99cSpec> {
        Gpio142readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO143 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio143read_privilege_of_master(
        &mut self,
    ) -> Gpio143readPrivilegeOfMasterW<Gpio99cSpec> {
        Gpio143readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#35\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio99c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio99c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio99cSpec;
impl crate::RegisterSpec for Gpio99cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio99c::R`](R) reader structure"]
impl crate::Readable for Gpio99cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio99c::W`](W) writer structure"]
impl crate::Writable for Gpio99cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO99C to value 0xffff_ffff"]
impl crate::Resettable for Gpio99cSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
