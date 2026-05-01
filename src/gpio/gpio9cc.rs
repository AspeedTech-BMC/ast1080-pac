#[doc = "Register `GPIO9CC` reader"]
pub type R = crate::R<Gpio9ccSpec>;
#[doc = "Register `GPIO9CC` writer"]
pub type W = crate::W<Gpio9ccSpec>;
#[doc = "Field `GPIO188ReadPrivilegeOfMaster` reader - GPIO188 Read Privilege of Master"]
pub type Gpio188readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO188ReadPrivilegeOfMaster` writer - GPIO188 Read Privilege of Master"]
pub type Gpio188readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO189ReadPrivilegeOfMaster` reader - GPIO189 Read Privilege of Master"]
pub type Gpio189readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO189ReadPrivilegeOfMaster` writer - GPIO189 Read Privilege of Master"]
pub type Gpio189readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO190ReadPrivilegeOfMaster` reader - GPIO190 Read Privilege of Master"]
pub type Gpio190readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO190ReadPrivilegeOfMaster` writer - GPIO190 Read Privilege of Master"]
pub type Gpio190readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO191ReadPrivilegeOfMaster` reader - GPIO191 Read Privilege of Master"]
pub type Gpio191readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO191ReadPrivilegeOfMaster` writer - GPIO191 Read Privilege of Master"]
pub type Gpio191readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO188 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio188read_privilege_of_master(&self) -> Gpio188readPrivilegeOfMasterR {
        Gpio188readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO189 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio189read_privilege_of_master(&self) -> Gpio189readPrivilegeOfMasterR {
        Gpio189readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO190 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio190read_privilege_of_master(&self) -> Gpio190readPrivilegeOfMasterR {
        Gpio190readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO191 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio191read_privilege_of_master(&self) -> Gpio191readPrivilegeOfMasterR {
        Gpio191readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO188 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio188read_privilege_of_master(
        &mut self,
    ) -> Gpio188readPrivilegeOfMasterW<Gpio9ccSpec> {
        Gpio188readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO189 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio189read_privilege_of_master(
        &mut self,
    ) -> Gpio189readPrivilegeOfMasterW<Gpio9ccSpec> {
        Gpio189readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO190 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio190read_privilege_of_master(
        &mut self,
    ) -> Gpio190readPrivilegeOfMasterW<Gpio9ccSpec> {
        Gpio190readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO191 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio191read_privilege_of_master(
        &mut self,
    ) -> Gpio191readPrivilegeOfMasterW<Gpio9ccSpec> {
        Gpio191readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#47\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio9ccSpec;
impl crate::RegisterSpec for Gpio9ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio9cc::R`](R) reader structure"]
impl crate::Readable for Gpio9ccSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio9cc::W`](W) writer structure"]
impl crate::Writable for Gpio9ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO9CC to value 0xffff_ffff"]
impl crate::Resettable for Gpio9ccSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
