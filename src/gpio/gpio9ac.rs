#[doc = "Register `GPIO9AC` reader"]
pub type R = crate::R<Gpio9acSpec>;
#[doc = "Register `GPIO9AC` writer"]
pub type W = crate::W<Gpio9acSpec>;
#[doc = "Field `GPIO156ReadPrivilegeOfMaster` reader - GPIO156 Read Privilege of Master"]
pub type Gpio156readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO156ReadPrivilegeOfMaster` writer - GPIO156 Read Privilege of Master"]
pub type Gpio156readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO157ReadPrivilegeOfMaster` reader - GPIO157 Read Privilege of Master"]
pub type Gpio157readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO157ReadPrivilegeOfMaster` writer - GPIO157 Read Privilege of Master"]
pub type Gpio157readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO158ReadPrivilegeOfMaster` reader - GPIO158 Read Privilege of Master"]
pub type Gpio158readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO158ReadPrivilegeOfMaster` writer - GPIO158 Read Privilege of Master"]
pub type Gpio158readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO159ReadPrivilegeOfMaster` reader - GPIO159 Read Privilege of Master"]
pub type Gpio159readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO159ReadPrivilegeOfMaster` writer - GPIO159 Read Privilege of Master"]
pub type Gpio159readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO156 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio156read_privilege_of_master(&self) -> Gpio156readPrivilegeOfMasterR {
        Gpio156readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO157 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio157read_privilege_of_master(&self) -> Gpio157readPrivilegeOfMasterR {
        Gpio157readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO158 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio158read_privilege_of_master(&self) -> Gpio158readPrivilegeOfMasterR {
        Gpio158readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO159 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio159read_privilege_of_master(&self) -> Gpio159readPrivilegeOfMasterR {
        Gpio159readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO156 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio156read_privilege_of_master(
        &mut self,
    ) -> Gpio156readPrivilegeOfMasterW<Gpio9acSpec> {
        Gpio156readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO157 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio157read_privilege_of_master(
        &mut self,
    ) -> Gpio157readPrivilegeOfMasterW<Gpio9acSpec> {
        Gpio157readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO158 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio158read_privilege_of_master(
        &mut self,
    ) -> Gpio158readPrivilegeOfMasterW<Gpio9acSpec> {
        Gpio158readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO159 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio159read_privilege_of_master(
        &mut self,
    ) -> Gpio159readPrivilegeOfMasterW<Gpio9acSpec> {
        Gpio159readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#39\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio9acSpec;
impl crate::RegisterSpec for Gpio9acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio9ac::R`](R) reader structure"]
impl crate::Readable for Gpio9acSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio9ac::W`](W) writer structure"]
impl crate::Writable for Gpio9acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO9AC to value 0xffff_ffff"]
impl crate::Resettable for Gpio9acSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
